use actix_web::{web, Responder};
use futures::StreamExt;
use bson::{doc, oid};
use super::super::{collection};

use crate::models::report::ReportData;

fn aggregate_report(_id: &str) -> Option<bson::ordered::OrderedDocument> {
  let coll = collection("reports");

  let mut stages: Vec<bson::Document> = vec![];

  let _match = doc! {
    "$match" => {
      "_id": oid::ObjectId::with_string(_id).unwrap()
    }
  };

  let lookup_course = doc! {
    "$lookup" => {
      "from" => "courses",
      "localField" => "course_id",
      "foreignField" => "_id",
      "as" => "course"
    }
  };

  let lookup_student = doc! {
    "$lookup" => {
      "from" => "students",
      "localField" => "student_id",
      "foreignField" => "_id",
      "as" => "student"
    }
  };

  let lookup_teacher = doc! {
    "$lookup" => {
      "from" => "teachers",
      "localField" => "teacher_id",
      "foreignField" => "_id",
      "as" => "teacher"
    }
  };

  let unwind_course = doc! {
    "$unwind" => {
      "path": "$course",
      "preserveNullAndEmptyArrays": true
    }
  };

  let unwind_student = doc! {
    "$unwind" => {
      "path": "$student",
      "preserveNullAndEmptyArrays": true
    }
  };

  let unwind_teacher = doc! {
    "$unwind" => {
      "path": "$teacher",
      "preserveNullAndEmptyArrays": true
    }
  };

  let project = doc! {
    "$project" => {
      "review_books": 0,
      "new_books": 0,
      "future_books": 0,
      "teacher.systemid": 0,
      "teacher.temporary": 0,
      "teacher.user_id": 0,
      "student.user_id": 0,
      "student.temporary": 0,
      "student.systemid": 0
    }
  };

  stages.push(_match);
  stages.push(lookup_student);
  stages.push(unwind_student);
  stages.push(lookup_teacher);
  stages.push(unwind_teacher);
  stages.push(lookup_course);
  stages.push(unwind_course);
  stages.push(project);

  let cursor = coll.aggregate(stages, None).unwrap();

  for result in cursor {
    if let Ok(document) = result {
      return Some(document);
    }
  }
  None
}

pub async fn get_reports() -> impl Responder {
  let coll = collection("reports");
  let cursor = coll.find(Some(doc!{}), None).unwrap();
  let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
  web::Json(docs)
}

pub async fn get_report(params: web::Path<(String,)>) -> impl Responder {
  if let Some(r) = aggregate_report(&params.0) {
    return web::Json(r);
  }

  web::Json(bson::ordered::OrderedDocument::new())
}


pub async fn create_report(mut body: web::Payload) -> Result<actix_web::HttpResponse, actix_web::Error> {

  let mut bytes = web::BytesMut::new();
  while let Some(item) = body.next().await {
    let item = item?;
    println!("Chuck: {:?}", &item);
    bytes.extend_from_slice(&item);
  }

  Ok(actix_web::HttpResponse::Ok().finish())
}