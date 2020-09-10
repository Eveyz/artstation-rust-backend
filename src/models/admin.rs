use serde::{Serialize, Deserialize};
use bson::{doc};
// use super::super::{collection};

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
  id: bson::oid::ObjectId, 
  username: String, 
  email: String, 
  identity: String, 
  status: String
}
