// use actix_web::{web, HttpRequest, Responder};
// use bson::{bson, doc, oid, Document};
// use mongodb::{options::{FindOneOptions}};
// use super::super::{collection};
// use std::string::String;
// use std::time::{Instant};
// use log::info;
// use serde_json::{Value};
// use chrono::{DateTime, Utc};

// use crate::models::schedule::Schedule;

// static NAME: &str = "schedules";

// pub async fn get_schedules() -> impl Responder {
//   let coll = collection(NAME);
//   let cursor = coll.find(Some(doc!{}), None).unwrap();
//   let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
//   web::Json(docs)
// }

// pub async fn get_schedule(params: web::Path<(String,)>) -> impl Responder {
//   let now = Instant::now();
//   let coll = collection(NAME);
//   let filter = Some(doc! { "_id": oid::ObjectId::with_string(&params.0).unwrap() });
//   let schedule = coll.find_one(filter, None).unwrap();
//   info!("find schedule time {}ms", now.elapsed().as_millis());
//   web::Json(schedule)
// }

// pub async fn create_schedule(schedule: web::Json<Schedule>) -> impl Responder {
//   let coll = collection(NAME);
//   info!("post request {:?}", schedule);
//   info!("post request {:?}", schedule.title);

//   let mut res = r#"
//     {
//       "status": 200,
//       "msg": "Added schedule successfully"
//     }
//   "#;

//   let doc = doc! {
//     "title": schedule.title.to_string(),
//     "category": schedule.category.to_string(),
//     "created_at": Utc::now(),
//     "updated_at": Utc::now(),
//   };
//   match coll.insert_one(doc, None) {
//     Ok(res) => info!("Added schedule successfully"),
//     Err(err) => {
//       info!("Failed to add schedule {}", err);
//       res = r#"
//         {
//           "status": 400,
//           "msg": "Failed to add schedule"
//         }
//       "#;
//     }
//   }


//   // let mut result = bson::from_bson(bson::Bson(1));
//   // let _schedule = schedule.clone();
//   // match bson::to_bson(&_schedule) {
//   //   Ok(doc_bson) => match doc_bson {
//   //     bson::Bson::Document(doc) => {
//   //       coll.insert_one(doc, None);
//   //     },
//   //     _ => {
//   //       info!("Failed to create document");
//   //     }
//   //   },
//   //   Err(err) => {
//   //     info!("Failed to create BSON");
//   //   }
//   // }

//   let v: Value = serde_json::from_str(res).unwrap();

//   web::Json(v)
// }

// pub async fn delete_schedule(params: web::Path<(String,)>) -> impl Responder {
//   web::Json(1)
// }