use actix_web::{web, Responder};
use bson::{doc};
use super::super::{collection};

pub async fn get_students() -> impl Responder {
  let coll = collection("students");
  let cursor = coll.find(Some(doc!{}), None).unwrap();
  let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
  web::Json(docs)
}
