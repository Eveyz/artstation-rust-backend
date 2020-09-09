use serde::{Serialize, Deserialize};
use bson::{doc, DateTime, document::Document};
use std::cell::RefCell;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Certificate {
  name: String,
  url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeacherData {
  pub firstname: String,
  pub lastname: String,
  pub englishname: String,
  pub level: Option<i32>,
  pub age: Option<i32>,
  pub birthday: Option<String>,
  pub gender: Option<String>,
  pub city: Option<String>,
}

impl TeacherData {
  pub fn to_bson_document(&self) -> Document {
    let mut doc = Document::new();
    doc.insert("firstname", &self.firstname);
    doc.insert("lastname", &self.lastname);
    doc.insert("englishname", &self.englishname);
    match self.level {
      Some(level) => { doc.insert("level", level); },
      None => {},
    }
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
pub struct Teacher {
	#[serde(rename = "_id")]
	pub _id: bson::oid::ObjectId,
  pub level: i32,
  pub status: String,
  pub consent: bool,
  pub certificates: RefCell<Vec<Option<Certificate>>>,
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
  pub created_at: Option<DateTime>,
  pub updated_at: Option<DateTime>,
}

// impl Teacher {
  // pub fn created() -> Teacher {
    // Test {vec: RefCell::new(Vec::new()) }
  // }

  // pub fn add(&self, value: i32){  
  //   self.vec.borrow_mut().push(value);
  // }
// }