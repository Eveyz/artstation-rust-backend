extern crate actix_web;
extern crate listenfd;
extern crate mongodb;

use listenfd::ListenFd;
use actix_web::{server, App, HttpRequest, Json, Result, http::Method, Responder};

use mongodb::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use serde::{Serialize, Deserialize};

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

fn index(req: &HttpRequest) -> Result<Json<User>> {
    Ok(Json(User {email: "saiop147@gmail.com".to_string(), username: "admin".to_string()}))
}

// fn get_reports(req: &HttpRequest) -> impl Responder {
//     let client = Client::connect("localhost", 27017)
//     .expect("Failed to initialize standalone client.");
//     let coll = client.db("lighters").collection("reports");
//     let mut cursor = coll.find(None, None).unwrap();
// }

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .prefix("/api")
            .resource("/", |r| r.f(index))  
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run();
}

// systemfd --no-pid -s http::3000 -- cargo watch -x run