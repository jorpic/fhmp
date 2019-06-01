
use std::{env, fs, process};
use std::sync::{Arc, Mutex};
use failure::{Context, Error, Fail};
use log::info;
use env_logger::Env;
use serde::Deserialize;
use rusqlite::{params, Connection};
use warp::{Filter, http::StatusCode, Reply, Rejection};
use warp::filters::BoxedFilter;


#[derive(Deserialize)]
struct Config {
    keys: Vec<String>,
    server_port: u16,
    database_file: String,
}

trait WithContext<T>
{
    fn context(self, msg: &str) -> Result<T, Context<&str>>;
}

impl<T, E> WithContext<T> for Result<T, E>
    where E: Fail
{
    fn context(self, msg: &str) -> Result<T, Context<&str>> {
        self.map_err(|e| e.context(msg))
    }
}


fn main() -> Result<(), Error> {
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


fn init_database(database_file: &str) -> Result<Connection, rusqlite::Error> {
    let db = Connection::open(database_file)?;
    db.execute(
        "create table if not exists notes(json text)",
        params![],
    )?;
    db.execute(
        "create table if not exists reviews(json text)",
        params![],
    )?;
    Ok(db)
}


fn init_api(conn: Connection) -> BoxedFilter<(impl Reply,)> {
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


type Db = Arc<Mutex<Connection>>;

fn get_notes(db: Db, key: String) -> Result<impl Reply, Rejection> {
    // FIXME: replace unwraps with rejections.
    let db = db.lock().unwrap();
    let mut from_notes = db.prepare_cached("SELECT json from notes").unwrap();
    let mut rows = from_notes.query(params![]).unwrap();
    let mut notes = Vec::new();
    loop {
        match rows.next().unwrap() {
            None => break,
            Some(row) => {
                let json = row.get(0).unwrap();
                notes.push(json);
            }
        }
    }
    let notes = serde_json::Value::Array(notes);

    let mut from_notes = db.prepare_cached("SELECT json from reviews").unwrap();
    let mut rows = from_notes.query(params![]).unwrap();
    let mut reviews = Vec::new();
    loop {
        match rows.next().unwrap() {
            None => break,
            Some(row) => {
                let json = row.get(0).unwrap();
                reviews.push(json);
            }
        }
    }
    let reviews = serde_json::Value::Array(reviews);
    Ok(warp::reply::json(&serde_json::json!({
        "notes": notes,
        "reviews": reviews
    })))
}


fn put_notes(db: Db, key: String,  data: Vec<usize>) -> Result<impl Reply, Rejection> {
    info!("put_notes: {}", key);
    Ok(StatusCode::BAD_REQUEST)
}
