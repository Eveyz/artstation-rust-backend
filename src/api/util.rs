use bson::{doc, Bson::Double, oid::ObjectId};
use log::info;
use super::super::{collection};
use serde_json::{Value, json};
use futures::stream::StreamExt;

pub fn get_system_id(number: i64) -> String {
  let sum = number + 10;
  let res: String = match sum {
    0..=99 => {
      format!("{}{}", 0, sum)
    },
    _ => format!("{}", sum)
  };
  return res;
}

pub fn get_report_credit(situation: &str) -> f64 {
  let res = match situation {
    "正常上课" | "平台赠课" | "学员上课时间后才请假或无故缺课(1个课时费)" | "老师迟到早退10分钟以内(需免费于当堂或下堂课补全课时才可得1个课时费, 但会影响薪资晋级)" | "代课(1个课时费)" | "小组课单个学员首次请假(学员付0.5课时费观看上课录屏, 老师照旧获1课时费)" | "小组课单个学员非首次请假(学员付1课时费观看上课录屏, 老师获1课时费)" => 1.0,
    "学员开课前2小时内才请假(0.5个课时费)" | "老师无故迟到10分钟以上20分钟以内并且课程依旧进行(0.5个课时费)" => 0.5,
    _ => 0.0
  };
  res
}

pub fn get_student_credit(situation: &str) -> f64 {
  let res = match situation {
    "正常上课" | "学员上课时间后才请假或无故缺课(1个课时费)" | "学员迟到(不必补全课时, 可按时下课, 1个课时费)" | "老师迟到早退10分钟以内(需免费于当堂或下堂课补全课时才可得1个课时费, 但会影响薪资晋级)" | "代课(1个课时费)" | "小组课单个学员非首次请假(学员付1课时费观看上课录屏, 老师获1课时费)" => 1.0,
    "学员开课前2小时内才请假(0.5个课时费)" | "老师无故迟到10分钟以上20分钟以内并且课程依旧进行(0.5个课时费)" | "小组课单个学员首次请假(学员付0.5课时费观看上课录屏, 老师照旧获1课时费)" => 0.5,
    _ => 0.0
  };
  res
}

pub async fn caculate_report_amount(situation: &str, teacher_id: &ObjectId, course_id: &ObjectId) -> (f64, f64) {
  let report_credit = get_report_credit(situation);
  let mut report_price = 0.0;
  
  let coll_teacher = collection("teachers");
  let coll_course = collection("courses");
  let coll_teacher_rate = collection("teacher_rates");

  // check if course and teacher co-exits
  let mut flag = false;

  // get course_type and course_level
  let course_res = coll_course.find_one(doc!{"_id": course_id}, None).await.unwrap();
  let mut course_type: Value = json!(null);
  let mut course_level: Value = json!(null);
  match course_res {
    Some(course_doc) => {
      course_type = course_doc.get("course_type").unwrap().clone().into();
      course_level = course_doc.get("course_level").unwrap().clone().into();
    },
    None => { flag = false; }
  }

  // get teacher level
  let teacher_res = coll_teacher.find_one(doc!{"_id": teacher_id}, None).await.unwrap();
  let mut level: Value = json!(null);
  match teacher_res {
    Some(teacher_doc) => {
      level = teacher_doc.get("level").unwrap().clone().into();
    },
    None => { flag = false; }
  }

  // check teacher rate fist
  let mut cursor = coll_teacher_rate.find(doc!{"_id": teacher_id}, None).await.unwrap();
  for result in cursor.next().await {
    if let Ok(item) = result {
      let ct: Value = item.get("course_type").unwrap().clone().into();
      let cl: Value = item.get("course_level").unwrap().clone().into();

      if course_type == ct && course_level == cl {
        match item.get("rate").unwrap() {
            Double(rate) => {
              report_price = *rate;      
            },
            _ => {}
        }
        break;
      } 
    }
  }
    
  if report_price == 0.0 && flag {
    let level_str = format!("{}级", level.as_str().unwrap());
    match collection("LevelSalary").find_one(doc!{
        "course_level": course_type.as_str().unwrap(), 
        "course_type": course_type.as_str().unwrap(),
        "level": level_str
      }, None).await.unwrap() {
      Some(ls_doc) => { 
        match ls_doc.get("rate").unwrap() {
          Double(rate) => { report_price = *rate },
          _ => {}
        }
      },
      None => { report_price = 0.0; }
    }
  }

  // return report_price and report_amount
  (report_price, report_price * report_credit)
}

pub async fn decrease_student_balacne(student_id: &ObjectId, course_id: &ObjectId, situation: &str) {
  let course = collection("courses").find_one(doc!{"_id": course_id}, None).await.unwrap();
  let student = collection("students").find_one(doc!{"_id": student_id}, None).await.unwrap();
  let mut course_rate = get_student_credit(situation);
  let mut tuition_amount = 0.0;
  
  match course {
    Some(doc) => {
      match doc.get("course_rate") {
        Some(res) => {
          if let Double(cr) = res {
            course_rate *= cr;
          }
        },
        None => course_rate = 0.0
      }
    },
    None => {
      return;
    }
  }

  match student {
    Some(doc) => {
      match doc.get("tuition_amount") {
        Some(res) => {
          if let Double(ta) = res {
            tuition_amount -= course_rate;
          }
        },
        None => tuition_amount = 0.0
      }
    },
    None => {
      return;
    }
  }
  match collection("students").find_one_and_update(doc!{"_id": student_id}, doc!{"tuition_amount": tuition_amount}, None).await {
    Ok(res) => {
      info!("Decrease stduent tuition successfully");
    },
    Err(err) => {
      info!("Fail to decrease stduent tuition {}", err);
    }
  }
}

pub async fn increase_student_balance(student_id: &ObjectId, course_id: &ObjectId, situation: &str) {
  let course = collection("courses").find_one(doc!{"_id": course_id}, None).await.unwrap();
  let student = collection("students").find_one(doc!{"_id": student_id}, None).await.unwrap();
  let mut course_rate = get_student_credit(situation);
  let mut tuition_amount = 0.0;
  
  match course {
    Some(doc) => {
      match doc.get("course_rate") {
        Some(res) => {
          if let Double(cr) = res {
            course_rate *= cr;
          }
        },
        None => course_rate = 0.0
      }
    },
    None => {
      return;
    }
  }

  match student {
    Some(doc) => {
      match doc.get("tuition_amount") {
        Some(res) => {
          if let Double(ta) = res {
            tuition_amount += course_rate;
          }
        },
        None => tuition_amount = 0.0
      }
    },
    None => {
      return;
    }
  }
  match collection("students").find_one_and_update(doc!{"_id": student_id}, doc!{"tuition_amount": tuition_amount}, None).await {
    Ok(res) => {
      info!("Increase stduent tuition successfully");
    },
    Err(err) => {
      info!("Fail to increase stduent tuition {}", err);
    }
  }
}

pub async fn add_to_paycheck(teacher_id: &ObjectId, student_id: &ObjectId, course_id: &ObjectId, report_id: &ObjectId, course_date: &str, amount: &f64) {
  let month = &course_date[0..8];
  let coll = collection("paychecks");
  match coll.find_one_and_update(doc!{
    "teacher_id": teacher_id, 
    "month": month, 
    "paid": false
  }, doc!{ "$inc": {"amount": amount} }, None).await {
    Ok(res) => {
      match res {
        Some(doc) => {
          info!("Add report to paycheck successfully");
        },
        None => {
          let paycheck_coll = collection("paychecks");
          match paycheck_coll.insert_one(doc!{
            "teacher_id": teacher_id,
            "student_id": student_id,
            "course_id": course_id,
            "month": month,
            "reports": [teacher_id],
            "memo": "老师工资",
            "amount": 0
          }, None).await {
            Ok(res) => { info!("Add report to paycheck successfully: {}", res.inserted_id); },
            Err(err) => { info!("Fail to create new paycheck: {}", err); }
          }
        }
      }
    },
    Err(err) => {
      info!("Fail to add report to paycheck: {}", err);
    }
  }
}

pub async fn remove_from_paycheck(teacher_id: &ObjectId, student_id: &ObjectId, course_id: &ObjectId, report_id: &ObjectId, course_date: &str, amount: &f64) {
  let month = &course_date[0..8];
  let coll = collection("paychecks");
  match coll.find_one_and_update(doc!{"teacher_id": teacher_id, "month": month, "paid": false}, doc!{ "$inc": {"amount": -amount}, "$pull": {"reports": report_id} }, None).await {
    Ok(res) => {
      match res {
        Some(doc) => {
          info!("Remove report from paycheck successfully");
        },
        None => {
          info!("Paycheck not found");
        }
      }
    },
    Err(err) => {
      info!("Fail to remove report from paycheck: {}", err);
    }
  }
}
