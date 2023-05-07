use chrono::{NaiveDateTime, DateTime, TimeZone, Offset, Local, NaiveDate};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDate,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    
}

impl User {
    pub fn default() -> User {
        let local: NaiveDate = Local::now().date_naive();
       
        User {
            id: rand::random::<i32>(),
            name: "".to_string(),
            created_at: local,
            password: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            address: "".to_string(),
        }
    }
    pub fn date_to_string(&self) -> String {
        self.created_at.to_string()
    }
}