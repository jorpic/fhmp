
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
    let api = init_api(db).with(warp::log(""));
    warp::serve(api).run(([127, 0, 0, 1], config.server_port));
    Ok(())
}


fn init_database(database_file: &str)
    -> Result<sqlite::Connection, sqlite::Error>
{
    let db = sqlite::open(database_file)?;
    db.execute(
        "create table if not exists notes(key text not null, json text not null)"
    )?;
    db.execute(
        "create table if not exists reviews(key text not null, json text not null)"
    )?;
    Ok(db)
}


fn init_api(conn: sqlite::Connection) -> BoxedFilter<(impl Reply,)> {
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
        .map_err(|e| warp::reject::custom(e))
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
    let reviews = &data["reviews"].as_array().unwrap();
    let notes = &data["notes"].as_array().unwrap();
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

trait WithContext<T>
{
    fn context(self, msg: &str) -> Result<T, failure::Context<&str>>;
}

impl<T, E> WithContext<T> for Result<T, E>
    where E: failure::Fail
{
    fn context(self, msg: &str) -> Result<T, failure::Context<&str>> {
        self.map_err(|e| e.context(msg))
    }
}

