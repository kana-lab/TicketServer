use std::env;
use std::sync::{Mutex, RwLock};
use actix_web::{web, get, post, App, HttpServer, Result, HttpResponse};
use actix_web::web::Data;
use log::debug;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
struct Event {
    id: Option<usize>,
    amount: usize,
    price: f64,
    event_name: String,
    image: String,
    explanation: String,
}

#[derive(Clone, Serialize, Debug)]
struct NFT {
    name: String,
    description: String,
    image: String,
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
            let mut stmt = con.prepare("select * from events").unwrap();
            stmt.query_map(params![], |row| {
                Ok(Event {
                    id: row.get(0).unwrap(),
                    amount: row.get(1).unwrap(),
                    price: row.get(2).unwrap(),
                    event_name: row.get(3).unwrap(),
                    image: row.get(4).unwrap(),
                    explanation: row.get(5).unwrap(),
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

    fn insert(&self, mut e: Event) {
        {
            let mut events = self.events.write().unwrap();
            e.id = Some(events.len());
            events.push(e.clone());
        }

        *self.events_json.write().unwrap() = serde_json::to_string::<Vec<Event>>(
            self.events.read().unwrap().as_ref()
        ).unwrap();

        self.con.lock().unwrap().execute(
            "insert into events values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![e.id, e.amount, e.price, e.event_name, e.image, e.explanation],
        ).unwrap();
    }

    fn get_nft(&self, event_id: usize, ticket_num: usize) -> Option<NFT> {
        let events = self.events.read().unwrap();
        if events.len() <= event_id { return None; }
        let Event { amount, event_name, image, explanation, .. } = events[event_id].clone();
        if ticket_num >= amount { return None; }
        Some(NFT {
            name: format!("{} #{}", event_name, ticket_num),
            description: explanation,
            image,
        })
    }
}

#[post("/new")]
async fn new_ticket(
    event: web::Json<Event>, client: Data<DBAdapter>,
) -> Result<HttpResponse> {
    debug!("{:?}", event);
    client.insert(event.clone());
    Ok(HttpResponse::Ok().finish())
}

#[get("/events")]
async fn get_events(
    client: Data<DBAdapter>
) -> HttpResponse {
    let json = client.get_events();
    HttpResponse::Ok().content_type("application/json").body(json)
}

#[get("/nft/{event_id}/{ticket_num}.json")]
async fn get_nft(
    client: Data<DBAdapter>, path: web::Path<(usize, usize)>,
) -> HttpResponse {
    let (event_id, ticket_num) = path.into_inner();
    match client.get_nft(event_id, ticket_num) {
        Some(nft) => HttpResponse::Ok().json(nft),
        None => HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // --- setup logger ---
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    debug!("debug for env_logger.");

    // --- setup database ---
    let client = Data::new(DBAdapter::new());

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(new_ticket)
            .service(get_events)
            .service(get_nft)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
