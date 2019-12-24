extern crate actix_web;
extern crate listenfd;
extern crate mongodb;
extern crate serde_json;

use log::info;

use listenfd::ListenFd;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

use lazy_static::lazy_static;
use mongodb::{Client, Collection, options::{ClientOptions, FindOptions}};
use bson::{doc, bson};
use serde::{Serialize, Deserialize};

mod common;

fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");

    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    Client::with_uri_str("mongodb://localhost:27017/").unwrap()
}

fn collection(coll_name: &str) -> Collection {
    MONGO.database("lighters").collection(coll_name)
}


// mod models;

#[derive(Serialize)]
struct User {
    email: String,
    username: String,
}

#[derive(Serialize)]
struct Report {
    tutor_comment: String,
    situation: String,
}

fn index() -> impl Responder {
    // format!("Hello {}! id:{}", info.1, info.0)
    "Hello from actix-web"
}

fn get_reports() -> HttpResponse {

    let coll = collection("reports");
    info!("coll name: {:?}", coll.name());
    info!("coll namespace: {:?}", coll.namespace());

    let rs = coll.count_documents(None, None);
    info!("count = {}", rs.unwrap());

    // let cursor = coll.find(Some(doc!{}), None).unwrap();
    // let result = cursor.map(|doc| doc).collect();

    // for result in cursor {
    //     match result {
    //         Ok(document) => {
    //             println!("document: {:?}", document)
    //         }
    //         Err(e) => println!("err: {:?}", e),
    //     }
    // }


    // let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    // let serialized = serde_json::to_string(&docs).unwrap();
    HttpResponse::Ok()
                .body("serialized")
}

fn main() {

    init_logger();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                web::scope("/reports")
                .route("/", web::get().to(get_reports))
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8080").unwrap()
    };
    info!("server is listening on 8080");
    server.run().unwrap();
}

// systemfd --no-pid -s http::8080 -- cargo watch -x run