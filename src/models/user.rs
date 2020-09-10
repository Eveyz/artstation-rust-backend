extern crate chrono;
extern crate pwhash;

use chrono::{Utc};
use serde::{Serialize, Deserialize};
use bson::{doc, DateTime, document::Document};
use super::super::{collection};
use log::info;
use jsonwebtoken::{encode, Header, EncodingKey};
use actix_web::{web};
use pwhash::bcrypt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  id: bson::oid::ObjectId, 
  username: String, 
  email: String, 
  identity: String, 
  status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthUser {
  pub username: String,
  pub password: String,
}

impl AuthUser {
  pub fn _login(&self) {

  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
  pub username: String,
  pub password: String,
  pub password_conformation: String
}

impl NewUser {
  pub async fn unique(&self) -> bool {
    let mut uniq = true;
    let coll = collection("users");
    match coll.find_one(Some(doc! { "username": &self.username }), None).await {
      Ok(doc) => {
        if let Some(_u) = doc {
          uniq = false
        }
      },
      Err(_err) => {}
    }
    uniq
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  #[serde(rename = "_id")]
  pub _id: bson::oid::ObjectId,
  pub status: String,
  pub admin_created: bool,
  pub admin: bool,
  pub remember: bool,
  pub consent: bool,
  pub identity: String,  
  pub email: String,
  pub temporary_password: String,
  pub username: String,
  pub password: String,
  pub password_conformation: String,
  pub projects_ids: Option<Vec<bson::oid::ObjectId>>,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

impl User {

  pub async fn create(user: web::Json<NewUser>) -> &'static str {
    let coll = collection("users");

    let hp = bcrypt::hash(&user.password).unwrap();

    let doc = doc! {
      "username": &user.username,
      "password": hp.clone(),
      "password_conformation": hp,
      "admin": false,
      "remember": false,
      "consent": false,
      "created_at": Utc::now(),
      "updated_at": Utc::now(),
    };

    let mut res = r#"
      {
        "status": 200,
        "msg": "Added user successfully"
      }
    "#;

    if !user.unique().await {
      res = r#"
        {
          "status": 200,
          "msg": "Username existed"
        }
      "#;
      return res;
    }
  
    match coll.insert_one(doc, None).await {
      Ok(_inserted_id) => info!("Added user successfully"),
      Err(err) => {
        info!("Failed to add user {}", err);
        res = r#"
          {
            "status": 400,
            "msg": "Failed to add user"
          }
        "#;
      }
    }
    res
  }

  pub fn create_token(&self) -> String {
    
    let my_claims = Claims {
      id: self._id.clone(),
      username: self.username.to_string(),
      email: self.email.to_string(), 
      identity: self.identity.to_string(), 
      status: self.status.to_string(),
    };

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();

    token
  }

  pub async fn get_identity_data(&self) -> Option<Document> {
    let mut identity_data = None;
    if self.identity == "teacher" {
      let coll = collection("teachers");
      identity_data = coll.find_one(Some(doc! { "user_id": &self._id }), None).await.unwrap();
    } else if self.identity == "student" {
      let coll = collection("students");
      identity_data = coll.find_one(Some(doc! { "user_id": &self._id }), None).await.unwrap();
    }
    identity_data
  }
}
