use actix_web::{web, Responder};
use bson::{doc};
use super::super::{collection};
use futures::stream::{StreamExt};

pub async fn get_transactions() -> impl Responder {
  let coll = collection("transactions");
  let cursor = coll.find(Some(doc!{}), None).await.unwrap();
  let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect().await;
  web::Json(docs)
}