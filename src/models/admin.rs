use serde::{Serialize, Deserialize};
use bson::{doc, UtcDateTime};
use json::{object};
use super::super::{collection};
use log::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
  id: bson::oid::ObjectId, 
  username: String, 
  email: String, 
  identity: String, 
  status: String
}
