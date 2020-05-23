use actix_web::{get, web, Responder};
use bson::{doc};
use super::super::{collection};

static NAME: &str = "teachers";

pub async fn get_teachers() -> impl Responder {
  let coll = collection(NAME);
  let cursor = coll.find(Some(doc!{}), None).unwrap();
  let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
  web::Json(docs)
}

pub async fn get_teacher(params: web::Path<(String)>) -> impl Responder {
  let coll = collection(NAME);
  web::Json(1)
}