use serde::{Serialize, Deserialize};
use bson::{DateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "_id")]
	pub _id: bson::oid::ObjectId,
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub position: i32,
    pub small_image_url: String,
    pub large_image_url: String,
    pub signed_original_image_url: String,
    pub viewport_constraint_type: String,
    pub asset_type: String,
    pub small_square_image_url: String,
    pub crop_x: i32,
    pub crop_y: i32,
    pub crop_w: i32,
    pub crop_h: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}