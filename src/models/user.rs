extern crate chrono;
extern crate pwhash;

use chrono::{Duration, Utc, DateTime};
use serde::{Serialize, Deserialize};
use bson::{doc, UtcDateTime};
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
  pub fn login(&self) {

  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
  pub username: String,
  pub password: String,
  pub password_conformation: String
}

impl NewUser {
  pub fn unique(&self) -> bool {
    let mut uniq = true;
    let coll = collection("users");
    match coll.find_one(Some(doc! { "username": &self.username }), None) {
      Ok(doc) => {
        if let Some(u) = doc {
          uniq = false
        }
      },
      Err(err) => {}
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
  pub created_at: UtcDateTime,
  pub updated_at: UtcDateTime,
}

impl User {

  pub fn create(user: web::Json<NewUser>) -> &'static str {
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

    if !user.unique() {
      res = r#"
        {
          "status": 200,
          "msg": "Username existed"
        }
      "#;
      return res;
    }
  
    match coll.insert_one(doc, None) {
      Ok(inserted_id) => info!("Added user successfully"),
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

  pub fn get_identity_data(&self) -> Option<bson::ordered::OrderedDocument> {
    let mut identity_data = None;
    if self.identity == "teacher" {
      let coll = collection("teachers");
      identity_data = coll.find_one(Some(doc! { "user_id": &self._id }), None).unwrap();
    } else if self.identity == "student" {
      let coll = collection("students");
      identity_data = coll.find_one(Some(doc! { "user_id": &self._id }), None).unwrap();
    }
    identity_data
  }
}
