use serde::{Serialize, Deserialize};
use bson::{UtcDateTime};
use std::cell::RefCell;

#[derive(Serialize, Deserialize)]
pub struct AudioFile {
    pub originalname: String,
    pub filename: String,
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReportData {
    pub teacher_id: bson::oid::ObjectId,
    pub course_id: bson::oid::ObjectId,
    pub student_id: bson::oid::ObjectId,
    pub situation: String,
    pub reason: Option<String>,
    pub course_date: String,
    pub duration: i64,
    pub focus: i64,
    pub tutor_comment: String,
    pub homework: String,
    pub start_time: String,
    pub end_time: String,
    pub external_link: Option<String>,
    pub audios_files: Option<RefCell<Vec<Option<AudioFile>>>>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Report {
    #[serde(rename = "_id")]
	pub _id: bson::oid::ObjectId,
    pub teacher_id: bson::oid::ObjectId,
    pub course_id: bson::oid::ObjectId,
    pub student_id: bson::oid::ObjectId,
    pub situation: String,
    pub reason: String,
    pub course_date: String,
    pub duration: i64,
    pub focus: i64,
    pub r#type: String,
    pub tutor_comment: String,
    pub homework: String,
    pub start_time: String,
    pub end_time: String,
    pub external_link: String,
    pub audios_files: RefCell<Vec<Option<AudioFile>>>,
    pub paid: bool,
    pub credit: i64,
    pub teacher_rate: i64,
    pub status: String,
    pub amount: i64,
    pub created_at: Option<UtcDateTime>,
    pub updated_at: Option<UtcDateTime>,
}