use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Report {
    tutor_comment: String,
    situation: String,
}