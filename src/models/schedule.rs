use serde::{Serialize, Deserialize};
use bson::{UtcDateTime};
use chrono::{Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
  #[serde(rename = "_id")]
  pub id: Option<bson::oid::ObjectId>,
	pub title: String,
  pub category: String,
  pub created_at: Option<UtcDateTime>,
  pub updated_at: Option<UtcDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableSchedule {
	pub title: String,
  pub category: String,
  pub created_at: UtcDateTime,
  pub updated_at: UtcDateTime,
}

// impl InsertableSchedule {
//   fn create_schedule(schedule: Schedule) -> InsertableSchedule {
//     InsertableSchedule {
//       title: schedule.title,
//       category: schedule.category,
//       created_at: Utc::now(),
//       updated_at: Utc::now(),
//     }
//   }
// }
