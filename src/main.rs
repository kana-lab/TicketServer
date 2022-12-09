use std::env;
use std::sync::{Mutex, RwLock};
use actix_cors::Cors;
use actix_web::{web, get, post, App, HttpServer, Result, HttpResponse};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use log::{debug, info};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
struct Event {
    event_name: String,
    image: String,
    explanation: String,
    address: String,
}

struct DBAdapter {
    // キャッシュ
    events: RwLock<Vec<Event>>,
    events_json: RwLock<String>,

    con: Mutex<Connection>,
}

impl DBAdapter {
    const DB_PATH: &'static str = "./identifier.sqlite";

    fn new() -> Self {
        let con = Connection::open(Self::DB_PATH).unwrap();

        let events = {
            let mut stmt = con.prepare("select * from stockInfo").unwrap();
            stmt.query_map(params![], |row| {
                Ok(Event {
                    event_name: row.get(0).unwrap(),
                    image: row.get(1).unwrap(),
                    explanation: row.get(2).unwrap(),
                    address: row.get(3).unwrap(),
                })
            }).unwrap().map(|e| e.unwrap()).collect()
        };
        debug!("{:?}", events);

        let events_json = serde_json::to_string(&events).unwrap();
        debug!("{:?}", events_json);

        Self {
            events: RwLock::new(events),
            events_json: RwLock::new(events_json),
            con: Mutex::new(con),
        }
    }

    fn get_events(&self) -> String {
        self.events_json.read().unwrap().clone()
    }

    fn insert(&self, e: Event) {
        self.events.write().unwrap().push(e.clone());

        *self.events_json.write().unwrap() = serde_json::to_string::<Vec<Event>>(
            self.events.read().unwrap().as_ref()
        ).unwrap();

        self.con.lock().unwrap().execute(
            "insert into stockInfo values (?1, ?2, ?3, ?4)",
            params![e.event_name, e.image, e.explanation, e.address],
        ).unwrap();
    }
}

#[post("/new")]
async fn new_ticket(
    event: web::Json<Event>, client: Data<DBAdapter>,
) -> Result<HttpResponse> {
    info!("{:?}", event);
    client.insert(event.clone());
    Ok(HttpResponse::Ok().finish())
}

#[get("/list")]
async fn get_events(
    client: Data<DBAdapter>
) -> HttpResponse {
    let json = client.get_events();
    HttpResponse::Ok().content_type("application/json").body(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // --- setup logger ---
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    debug!("debug for env_logger.");

    // --- setup database ---
    let client = Data::new(DBAdapter::new());

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(client.clone())
            .service(new_ticket)
            .service(get_events)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
