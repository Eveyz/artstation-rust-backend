use actix_web::{web, Responder};
use bson::{doc};
use super::super::{collection};
use log::info;
use serde_json::{Value, json};
// use chrono::{Utc};
// use futures::stream::StreamExt;

use crate::models::user::{User, AuthUser, NewUser};
use bcrypt::verify;

static NAME: &str = "users";

pub async fn authenticate(auth_user: web::Json<AuthUser>) -> impl Responder {
  // user login info missing
  let mut res = r#"
    {
      "status": 200,
      "msg": "User info missing"
    }
  "#;
  let v: Value = serde_json::from_str(res).unwrap();
  if auth_user.username.is_empty() && auth_user.password.is_empty() {
    return web::Json(v);
  }

  let coll = collection(NAME);
  match coll.find_one(Some(doc!{ "username": &auth_user.username }), None).await {
    Ok(result) => {
      match result {
        Some(_user) => {
          // info!("{:?}", _user);
          let user: User = bson::from_bson(bson::Bson::Document(_user)).unwrap();
          let valid = verify(&auth_user.password, &user.password).unwrap();
          if valid {
            info!("valid password");
            let response = json!({
              "token": user.create_token(),
              "identity": user.get_identity_data().await
            });
            return web::Json(response);
          } else {
            info!("invalid password");
            res = r#"
              {
                "status": 404,
                "msg": "Wrong password"
              }
            "#;
          }
        },
        None => {
          info!("user not found");
          res = r#"
            {
              "status": 400,
              "msg": "User not found"
            }
          "#;
        }
      }
    },
    Err(err) => {
      info!("failed to find user {}", err);
      res = r#"
        {
          "status": 400,
          "msg": "Fail to authenticate user"
        }
      "#;
    }
  }
  
  let v: Value = serde_json::from_str(res).unwrap();
  web::Json(v)
}

pub async fn create_user(user: web::Json<NewUser>) -> impl Responder {
  // let coll = collection(NAME);

  let mut _res = r#"
    {
      "status": 200,
      "msg": "Added schedule successfully"
    }
  "#;

  if user.password != user.password_conformation {
    _res = r#"
      {
        "status": 400,
        "msg": "Password not matched"
      }
    "#;
  } else {
    _res = User::create(user).await;
  }

  let v: Value = serde_json::from_str(_res).unwrap();

  web::Json(v)
}

// pub async fn get_users() -> impl Responder {
//   let coll = collection(NAME);
//   let cursor = coll.find(Some(doc!{}), None).await.unwrap();
//   let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect().await;
//   web::Json(docs)
// }

// pub async fn get_user(params: web::Path<(String,)>) -> impl Responder {
//   let now = Instant::now();
//   let coll = collection(NAME);
//   let filter = Some(doc! { "_id": oid::ObjectId::with_string(&params.0).unwrap() });
//   let user = coll.find_one(filter, None).await.unwrap();
//   info!("find user time {}ms", now.elapsed().as_millis());
//   web::Json(user)
// }

// pub async fn delete_user(params: web::Path<(String,)>) -> impl Responder {
//   web::Json(1)
// }