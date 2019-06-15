
// TODO:
//  - reject if unknow key
//  - rsynk-like exchange protocol to save some traffic

use std::{env, fs, process};
use std::sync::{Arc, Mutex};
use failure::{Error, Fallible, err_msg};
use log::info;
use env_logger::Env;
use serde::Deserialize;
use warp::{Filter, Reply, Rejection};
use warp::filters::BoxedFilter;


#[derive(Deserialize)]
struct Config {
    keys: Vec<String>,
    server_port: u16,
    allow_origin: String,
    database_file: String,
}


fn main() -> Fallible<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: sync-server <config.toml>");
        process::exit(1);
    }
    let config_toml = fs::read_to_string(&args[1])
        .context("Unable to read config file")?;
    let config: Config = toml::from_str(&config_toml)
        .context("Unable to parse config file")?;

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let db = init_database(&config.database_file)
        .context("Unable to open or init database")?;
    let api = init_api(db, &config).with(warp::log(""));
    warp::serve(api).run(([127, 0, 0, 1], config.server_port));
    Ok(())
}


fn init_database(database_file: &str)
    -> Result<sqlite::Connection, sqlite::Error>
{
    let db = sqlite::open(database_file)?;
    db.execute(
        "create table if not exists notes(
            key text not null,
            json text not null);
        create unique index if not exists notes_ix
            on notes(key, json_extract(json, '$.id'));
        create table if not exists reviews(
            key text not null,
            json text not null);
        create unique index if not exists reviews_ix
            on reviews(key, json_extract(json, '$.id'));"
    )?;
    Ok(db)
}


fn init_api(conn: sqlite::Connection, config: &Config) -> BoxedFilter<(impl Reply,)> {
    let db = Arc::new(Mutex::new(conn));
    let db = warp::any().map(move || db.clone());
    let put = warp::post2()
        .and(db.clone())
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(put_notes);
    let get = warp::get2()
        .and(db.clone())
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_notes);
    let cors = warp::cors()
        .allow_origin(config.allow_origin.as_str())
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);
    put.or(get).with(cors).boxed()
}


type Db = Arc<Mutex<sqlite::Connection>>;

fn get_notes(db: Db, key: String) -> Result<impl Reply, Rejection> {
    get_notes_aux(db, key)
        .map_err(warp::reject::custom)
}

fn get_notes_aux(db: Db, key: String) -> Fallible<impl Reply> {
    let notes = select_json(
        &db,
        "SELECT json from notes where key = ?",
        key.as_str())?;
    let reviews = select_json(
        &db,
        "SELECT json from reviews where key = ?",
        key.as_str())?;

    Ok(warp::reply::json(&serde_json::json!({
        "notes": notes,
        "reviews": reviews,
    })))
}


fn put_notes(db: Db, key: String,  data: serde_json::Value) -> Result<impl Reply, Rejection> {
    put_notes_aux(db, key, data)
        .map_err(warp::reject::custom)
}

fn put_notes_aux(db: Db, key: String, data: serde_json::Value) -> Fallible<impl Reply> {
    let reviews = &data["reviews"].as_array().unwrap(); // FIXME: .ok_or(err)?;
    let notes = &data["notes"].as_array().unwrap();

    let db = db.lock()
        .map_err(|_| err_msg("db.lock() failed"))?;
    let mut q = db.prepare("insert or ignore into reviews values (?, ?)")?;
    // NB. We need to filter out reviews and notes without `id`
    // because unique index does not work if `json_extract(..) == NULL`.
    // FIXME: Should we drop an error in case we found such a review?
    for r in reviews.iter().filter(|r| r["id"].as_str().is_some()) {
        q.reset()?;
        q.bind(1, &key[..])?;
        q.bind(2, &r.to_string()[..])?;
        while let sqlite::State::Row = q.next()? { };
    }

    let mut q = db.prepare("
        insert or replace into notes (key, json)
            values (
                ?,
                coalesce(
                    (select json from notes
                        where key = ?
                          and json_extract(json, '$.id') = ?
                          and json_extract(json, '$.ver') >= ?),
                    ?)
            )")?;
    for n in notes.iter() {
        if let Some(id) = n["id"].as_str() {
            if let Some(ver) = n["ver"].as_str() {
                q.reset()?;
                q.bind(1, &key[..])?;
                q.bind(2, &key[..])?;
                q.bind(3, &id[..])?;
                q.bind(4, &ver[..])?;
                q.bind(5, &n.to_string()[..])?;
                while let sqlite::State::Row = q.next()? { };
            }
        }
    }

    Ok(warp::reply())
}

// Helper stuff
fn select_json(db: &Db, query: &str, key: &str)
    -> Fallible<serde_json::Value>
{
    let db = db.lock()
        .map_err(|_| err_msg("db.lock() failed"))?;
    let mut from_notes = db.prepare(query)?;
    from_notes.bind(1, key)?;
    let mut values = Vec::new();
    while let sqlite::State::Row = from_notes.next()? {
        let text = from_notes.read::<String>(0)?;
        let json = serde_json::from_str(&text)?;
        values.push(json);
    }
    Ok(serde_json::Value::Array(values))
}

trait WithContext<T> {
    fn context(self, msg: &str) -> Result<T, failure::Context<&str>>;
}

impl<T, E> WithContext<T> for Result<T, E>
    where E: failure::Fail
{
    fn context(self, msg: &str) -> Result<T, failure::Context<&str>> {
        self.map_err(|e| e.context(msg))
    }
}

