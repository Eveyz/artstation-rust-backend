// use actix_web::{get, web, Responder};
// use bson::{doc, oid};
// use log::info;
// use super::super::{collection};
// use futures::stream::{StreamExt};

// use crate::models::teacher::{Teacher};

// static NAME: &str = "teachers";

// pub async fn get_teachers() -> impl Responder {
//   let coll = collection(NAME);
//   let cursor = coll.find(Some(doc!{}), None).await.unwrap();
//   let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect().await;
//   web::Json(docs)
// }

// pub async fn get_teacher(params: web::Path<(String,)>) -> impl Responder {
//   let coll = collection(NAME);
//   info!("teacher id: {:?}", params);
  
//   let mut stages: Vec<bson::Document> = vec![];

//   let _match = doc! {
//     "$match": {
//       "_id": oid::ObjectId::with_string(&params.0).unwrap()
//     }
//   };

//   let lookup = doc! {
//     "$lookup": {
//       "from": "users",
//       "localField": "user_id",
//       "foreignField": "_id",
//       "as": "user"
//     }
//   };

//   let unwind = doc! {
//     "$unwind": "$user"
//   };

//   let project = doc! {
//     "$project": {
//       "certificates": 0,
//       "user.temporaryPassword": 0,
//       "user.password": 0,
//       "user.passwordCon": 0
//     }
//   };

//   stages.push(_match);
//   stages.push(lookup);
//   stages.push(unwind);
//   stages.push(project);

//   let mut cursor = coll.aggregate(stages, None).await.unwrap();

//   for result in cursor.next().await {
//     if let Ok(document) = result {
//       return web::Json(document)
//     }
//   }

//   web::Json(bson::document::Document::new())
// }
