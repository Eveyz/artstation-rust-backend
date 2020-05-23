use actix_web::{web, Responder};
use bson::{doc};
use super::super::{collection};

pub async fn get_reports() -> impl Responder {
  let coll = collection("reports");
  let cursor = coll.find(Some(doc!{}), None).unwrap();
  let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
  web::Json(docs)
}


// async fn get_reports_sample() -> impl Responder {

//   let now = Instant::now();

//   let coll = collection("students");
//   info!("coll name: {:?}", coll.name());
//   info!("coll namespace: {:?}", coll.namespace());
//   info!("coll time {}", now.elapsed().as_millis());

//   // let count_options = EstimatedDocumentCountOptions::builder().build();
//   let rs = coll.estimated_document_count(None);
//   match rs {
//       Ok(count) => info!("count = {}", count),
//       Err(e) => info!("Err message is here: {}", e),
//   }

//   let cursor = coll.find(Some(doc!{}), None).unwrap();
//   // let results: Vec<Result<_>> = cursor.collect();

//   // for result in cursor {
//   //     match result {
//   //         Ok(document) => {
//   //             println!("document: {:?}", document)
//   //         }
//   //         Err(e) => println!("err: {:?}", e),
//   //     }
//   // }

//   let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
//   info!("vec time {}", now.elapsed().as_millis());

//   // let serialized = serde_json::to_string(&docs).unwrap();
//   // info!("serialized time {}", now.elapsed().as_millis());
  
//   web::Json(docs)
//   // HttpResponse::Ok()
//               // .body("serialized")
// }
