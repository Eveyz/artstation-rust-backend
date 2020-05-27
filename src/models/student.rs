use serde::{Serialize, Deserialize};
use bson::{doc, UtcDateTime};
use bson::ordered::OrderedDocument;
use std::cell::RefCell;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentData {
  pub firstname: String,
  pub lastname: String,
  pub englishname: String,
  pub tuition_amount: i32,
  pub age: Option<i32>,
  pub birthday: Option<String>,
  pub gender: Option<String>,
  pub city: Option<String>,
}

impl StudentData {
  pub fn to_bson_document(&self) -> OrderedDocument {
    let mut doc = OrderedDocument::new();
    doc.insert("firstname", &self.firstname);
    doc.insert("lastname", &self.lastname);
    doc.insert("englishname", &self.englishname);
    match self.age {
      Some(age) => { doc.insert("age", age); },
      None => {},
    }
    match &self.birthday {
      Some(birthday) => { doc.insert("birthday", birthday); },
      None => {},
    }
    match &self.gender {
      Some(gender) => { doc.insert("gender", gender); },
      None => {},
    }
    match &self.city {
      Some(city) => { doc.insert("city", city); },
      None => {},
    }
    doc
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student {
	#[serde(rename = "_id")]
	pub _id: bson::oid::ObjectId,
  pub level: i32,
  pub status: String,
  pub consent: bool,
  pub courses: RefCell<Vec<bson::oid::ObjectId>>,
  pub students: RefCell<Vec<bson::oid::ObjectId>>,
  pub firstname: String,
  pub lastname: String,
  pub englishname: String,
  pub age: i32,
  pub birthday: String,
  pub gender: String,
  pub city: String,
  pub user_id: bson::oid::ObjectId,
  pub systemid: String,
  pub temporary: String,
  pub title: Option<String>,
  pub description: Option<String>,
  pub author: Option<String>,
  pub work: Option<String>,
  pub education: Option<String>,
  pub experience: Option<String>,
  pub otherexperience: Option<String>,
  pub profour: Option<i32>,
  pub proeight: Option<i32>,
  pub levelsix: Option<i32>,
  pub other: Option<String>,
  pub honor: Option<String>,
  pub interaction: Option<String>,
  pub like: Option<String>,
  pub availabletime: Option<i32>,
  pub audio: Option<String>,
  pub comments: Option<String>,
  pub resume: Option<String>,
  pub rate: Option<i32>,
  pub created_at: Option<UtcDateTime>,
  pub updated_at: Option<UtcDateTime>,
}