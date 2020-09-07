use actix_web::{web, Responder};
use bson::{doc, oid};
use super::super::{collection};
use log::info;

use serde_json::{Value, json};
use crate::models::report::ReportData;
use crate::api::util::{caculate_report_amount, increase_student_balance, decrease_student_balacne, add_to_paycheck, remove_from_paycheck};

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


pub async fn create_report(report_data: web::Json<ReportData>) -> impl Responder {
  
  let coll = collection("reports_test");

  let mut response = bson::ordered::OrderedDocument::new();
  response.insert("status", 200);
  response.insert("msg", "success");

  let (report_price, report_amount) = caculate_report_amount(&report_data.situation, &report_data.teacher_id, &report_data.course_id);

  match coll.insert_one(report_data.to_bson_document(report_price, report_amount), None) {
    Ok(res) => {
      match res.inserted_id {
        bson::Bson::String(report_id) => {

          // created report successfully
          let report_object_id = bson::oid::ObjectId::with_string(&report_id).unwrap();
          
          // decrease student balance
          decrease_student_balacne(&report_data.student_id, &report_data.course_id, &report_data.situation);
          
          // add to teacher paycheck
          add_to_paycheck(&report_data.teacher_id, &report_data.student_id, &report_data.course_id, &report_object_id, &report_data.course_date, &report_amount);

          match aggregate_report(&report_id[..]) {
            Some(report) => {

              return web::Json(report);
            },
            None => {}
          }
        },
        _ => {}
      }
    },
    Err(err) => {
      info!("err when creating report: {}", err);
      response.insert("msg", "Fail to create report");
    }
  }

  web::Json(response)
}
