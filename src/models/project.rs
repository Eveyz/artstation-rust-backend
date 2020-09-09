extern crate chrono;
use serde::{Serialize, Deserialize};
use bson::{doc, DateTime};
use super::super::{collection};
use log::info;
use actix_web::{web};
use std::cell::RefCell;
use super::asset::Asset;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
  pub id: bson::oid::ObjectId, 
  pub title: String, 
  pub description: String, 
  pub description_text: String, 
  pub permalink: String,
  pub published_at: String,
  pub slug: String,
  pub hash_id: String,
  pub tags: Vec<String>,
  pub assets: RefCell<Vec<Option<Asset>>>,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}