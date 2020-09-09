use actix_web::{get, web, Responder};
use bson::{doc, oid, DateTime};
use log::info;
use super::super::{collection};
use chrono::{Datelike, Utc};
use serde_json::{Value};

use crate::models::teacher::TeacherData;
use crate::models::student::StudentData;
use crate::api::util::get_system_id;

pub async fn create_teacher(teacher_data: web::Json<TeacherData>) -> impl Responder {

  let mut response = r#"
    {
      "status": 200,
      "msg": "Success"
    }
  "#;

  let coll = collection("teachers");
  let count = coll.estimated_document_count(None).await.unwrap();
  let system_id = get_system_id(count);
  let current_year = Utc::now().date().year() / 100;
  let teacher_user_name = format!("T{}{}", current_year, system_id);
  let email = format!("{}@lighters.com", teacher_user_name);

  let user_doc = doc! {
    "identity": "teacher".to_string(),
    "username": teacher_user_name.clone(),
    "email": email,
    "temporaryPassword": teacher_user_name.clone(),
    "password": teacher_user_name.clone(),
    "passwordCon": teacher_user_name.clone(),
    "adminCreated": true,
    "status": "RESET_REQUIRED".to_string(),
    "admin": false,
    "consent": false,
    "remember": false,
    "created_at": Utc::now(),
    "updated_at": Utc::now(),
  };

  
  let coll_user = collection("user_test");
  match coll_user.insert_one(user_doc, None).await {
      Ok(res) => {

          let mut teacher = teacher_data.to_bson_document();
          teacher.insert("user_id", res.inserted_id);
          teacher.insert("status", "active");
          teacher.insert("systemid", teacher_user_name.clone());
          teacher.insert("temporary", teacher_user_name);
          teacher.insert("created_at", Utc::now());
          teacher.insert("updated_at", Utc::now());
          info!("teacher {}", teacher);

          let coll_teacher = collection("teacher_test");

          match coll_teacher.insert_one(teacher, None).await {
            Ok(res) => {
              info!("Created teacher succesfully");
              response = r#"
                {
                  "status": 200,
                  "msg": "Created teacher succesfully"
                }
              "#;
            },
            Err(err) => {
              info!("err when creating user: {}", err);
              response = r#"
                {
                  "status": 200,
                  "msg": "Fail to create teacher"
                }
              "#;
            }
          }
    },
    Err(err) => {
      info!("err when creating user: {}", err);
      response = r#"
        {
          "status": 200,
          "msg": "Fail to create user"
        }
      "#;
    }
  }

  let v: Value = serde_json::from_str(response).unwrap();
  web::Json(v)
}

pub async fn create_student(student_data: web::Json<StudentData>) -> impl Responder {

  let mut response = r#"
    {
      "status": 200,
      "msg": "Success"
    }
  "#;

  let coll = collection("students");
  let count = coll.estimated_document_count(None).await.unwrap();
  let system_id = get_system_id(count);
  let current_year = Utc::now().date().year() / 100;
  let student_user_name = format!("S{}{}", current_year, system_id);
  let email = format!("{}@lighters.com", student_user_name);

  let user_doc = doc! {
    "identity": "student".to_string(),
    "username": student_user_name.clone(),
    "email": email,
    "temporaryPassword": student_user_name.clone(),
    "password": student_user_name.clone(),
    "passwordCon": student_user_name.clone(),
    "adminCreated": true,
    "status": "RESET_REQUIRED".to_string(),
    "admin": false,
    "consent": false,
    "remember": false,
    "created_at": Utc::now(),
    "updated_at": Utc::now(),
  };

  
  let coll_user = collection("user_test");
  match coll_user.insert_one(user_doc, None).await {
      Ok(res) => {

          let mut student = student_data.to_bson_document();
          student.insert("user_id", res.inserted_id);
          student.insert("status", "active");
          student.insert("systemid", student_user_name.clone());
          student.insert("temporary", student_user_name);
          student.insert("created_at", Utc::now());
          student.insert("updated_at", Utc::now());
          info!("student {}", student);

          let coll_student = collection("student_test");

          match coll_student.insert_one(student, None).await {
            Ok(res) => {
              info!("Created student succesfully");
              response = r#"
                {
                  "status": 200,
                  "msg": "Created student succesfully"
                }
              "#;
            },
            Err(err) => {
              info!("err when creating user: {}", err);
              response = r#"
                {
                  "status": 200,
                  "msg": "Fail to create student"
                }
              "#;
            }
          }
    },
    Err(err) => {
      info!("err when creating user: {}", err);
      response = r#"
        {
          "status": 200,
          "msg": "Fail to create user"
        }
      "#;
    }
  }

  let v: Value = serde_json::from_str(response).unwrap();
  web::Json(v)
}