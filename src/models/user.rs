use serde::{Serialize, Deserialize};
use bson::{doc, UtcDateTime};
use json::{object};
use super::super::{collection};
use log::info;
use jsonwebtoken::{encode, Header, EncodingKey};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  id: bson::oid::ObjectId, 
  username: String, 
  email: String, 
  identity: String, 
  status: String
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
pub struct User {
	#[serde(rename = "_id")]
	pub _id: bson::oid::ObjectId,
  pub status: String,
  pub adminCreated: bool,
  pub admin: bool,
  pub remember: bool,
  pub consent: bool,
  pub identity: String,  
	pub username: String,
	pub email: String,
  pub temporaryPassword: String,
  pub password: String,
  pub passwordCon: String,
  pub created_at: UtcDateTime,
  pub updated_at: UtcDateTime,
}

impl User {
  pub fn create_token(&self) -> String {
    
    let my_claims = Claims {
      id: self._id.clone(),
      username: self.username.to_string(),
      email: self.email.to_string(), 
      identity: self.identity.to_string(), 
      status: self.status.to_string()
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
