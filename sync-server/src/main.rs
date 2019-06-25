
// TODO:
//  - reject if unknow key
//  - rsynk-like exchange protocol to save some traffic

use std::{env, fs, process};
use std::sync::{Arc, Mutex};
use failure::{Error, Fallible, err_msg};
use log::info;
use env_logger::Env;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use warp::{Filter, Reply, Rejection};
use warp::filters::BoxedFilter;


#[derive(Deserialize)]
struct Config {
    keys: Vec<String>,
    server_port: u16,
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
    put.or(get).boxed()
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



fn put_notes(db: Db, key: String,  data: Value) -> Result<impl Reply, Rejection> {
    put_notes_aux(db, key, data)
        .map_err(warp::reject::custom)
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Note {
    id: String,
    text: String,
    ver: String,
    lastReview: String,
    nextReview: String,
}

#[derive(Serialize, Deserialize)]
struct Payload {
    reviews: Vec<Value>,
    notes: Vec<Note>,
}

fn put_notes_aux(db: Db, key: String, data: Value) -> Fallible<impl Reply> {
    let Payload {reviews, notes} = serde_json::from_value(data)?;

    let db = db.lock()
        .map_err(|_| err_msg("db.lock() failed"))?;
    let mut q = db.prepare("insert or ignore into reviews values (?, ?)")?;
    for x in reviews.iter() {
        q.reset()?;
        q.bind(1, &key[..])?;
        q.bind(2, &x.to_string()[..])?;
        while let sqlite::State::Row = q.next()? { };
    }

    let mut select = db.prepare(
        "select json from notes
            where key = ?
              and json_extract(json, '$.id') = ?"
    )?;
    let mut insert = db.prepare(
        "insert or replace into notes (key, json) values (?, ?)"
    )?;

    for x in notes.iter() {
        select.reset()?;
        select.bind(1, &key[..])?;
        select.bind(2, &x.id[..])?;
        while let sqlite::State::Row = select.next()? {
            let json = select.read::<String>(0)?;
            let mut note: Note = serde_json::from_str(&json)?;
            let mut needs_update = false;
            if note.ver < x.ver {
                note.text = x.text.clone();
                note.ver = x.ver.clone();
                needs_update = true;
            }
            if note.lastReview < x.lastReview {
                note.nextReview = x.nextReview.clone();
                note.lastReview = x.lastReview.clone();
                needs_update = true;
            }
            if needs_update {
                let json = serde_json::to_string(&note)?;
                insert.reset()?;
                insert.bind(1, &key[..])?;
                insert.bind(2, &json[..])?;
                while let sqlite::State::Row = insert.next()? { }
            }
        };
    }

    Ok(warp::reply())
}



// Helper stuff
fn select_json(db: &Db, query: &str, key: &str) -> Fallible<Value> {
    let db = db.lock()
        .map_err(|_| err_msg("db.lock() failed"))?;
    let mut q = db.prepare(query)?;
    q.bind(1, key)?;
    let mut values = Vec::new();
    while let sqlite::State::Row = q.next()? {
        let text = q.read::<String>(0)?;
        let json = serde_json::from_str(&text)?;
        values.push(json);
    }
    Ok(Value::Array(values))
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
