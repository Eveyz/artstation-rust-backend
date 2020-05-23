use actix_web::{web, HttpRequest, Responder};
use bson::{bson, doc, oid, Document};
use mongodb::{options::{FindOneOptions}};
use super::super::{collection};
use std::string::String;
use std::time::{Instant};
use log::info;

// use crate::models::schedule::Schedule;

static NAME: &str = "tuitions";

pub async fn get_tuitions() -> impl Responder {
  let coll = collection(NAME);
  let cursor = coll.find(Some(doc!{}), None).unwrap();
  let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
  web::Json(docs)
}