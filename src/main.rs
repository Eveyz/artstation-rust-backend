extern crate actix_web;
extern crate listenfd;
extern crate mongodb;
extern crate serde_json;
extern crate dotenv;
extern crate bcrypt;

use log::info;
use listenfd::ListenFd;
use actix_web::{web, App, HttpServer};
use actix_web::http::{header::CONTENT_TYPE, HeaderValue};
use actix_service::Service;
use lazy_static::lazy_static;
use mongodb::{Client, Collection, options::{FindOptions}};
use bson::{doc};
use dotenv::dotenv;
// use futures::future::FutureExt;

mod common;
mod api;
mod models;

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
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Client::with_uri_str(&database_url).unwrap()
}

fn collection(coll_name: &str) -> Collection {
    MONGO.database("artstation").collection(coll_name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let start = std::time::Instant::now();
    let mut sum = 0;
    for i in 0..100000000 {
        sum += i;
    }
    let duration = start.elapsed();
    info!("time elapsed {:?}", duration);

    // let HOST = dotenv::var("HOST").unwrap();
    // let PORT = dotenv::var("PORT").unwrap();
    let address = dotenv::var("ADDR").unwrap();

    init_logger();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(greet))
            .service(
                web::scope("/admin")
                .route("/create_teacher", web::post().to(api::admin::create_teacher))
                .route("/create_student", web::post().to(api::admin::create_student))
            )
            .service(
                web::scope("/users")
                .wrap_fn(|req, srv| {
                    info!("Hi this is from start. You requested {}", req.path());
                    info!("Hi this is from start. You requested {:?}", req.headers());

                    let headers = req.headers();
                    if let Some(auth) = headers.get("Authorization") {
                        info!("token: {}", auth.to_str().unwrap());
                    };

                    let fut = srv.call(req);
                    async {
                        let mut res = fut.await?;
                    //     res.headers_mut().insert(
                    //             CONTENT_TYPE, HeaderValue::from_static("application/json"),
                    //     );
                        Ok(res)
                    }
                })
                .route("/login", web::post().to(api::users::authenticate))
                .route("/signup", web::post().to(api::users::create_user))
            )
            .service(
                web::scope("/artworks")
                .route("", web::get().to(api::teachers::get_teachers))
                .route("/{_id}", web::get().to(api::teachers::get_teacher))
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(&address).unwrap()
    };
    info!("server is listening on {}", address);
    return server.run().await;
}

// systemfd --no-pid -s http::8080 -- cargo watch -x run