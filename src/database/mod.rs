use chrono::NaiveDate;
use chrono::{Local, DateTime};
use rusqlite::{Connection, Result};
use crate::schemas::User;

pub struct DataBase {
    pub conn: Connection,
    pub users: Vec<User>,
    pub count: i32,
    pub last_insert_id: i32,
    pub last_update_id: i32,
    pub last_delete_id: i32,
    pub fake_user: User
}
impl DataBase {
    pub fn create_table(&self) {
        self.conn.execute(
            "CREATE TABLE User (
                id   INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                password TEXT NOT NULL,
                email TEXT NOT NULL,
                phone TEXT NOT NULL,
                address TEXT NOT NULL
            )",
            (), // empty list of parameters.
        ).unwrap();
    }
    pub fn new()-> Self {
        let conn = Connection::open_in_memory().unwrap();
        Self {
            conn,
            users: Vec::new(),
            count: 0,
            last_insert_id: 0,
            last_update_id: 0,
            last_delete_id: 0,
            fake_user : User::default()
        }
    }
    pub fn insert_user(&self, user: User)-> Result<()> {
        self.conn.execute(
            "INSERT INTO User (name,created_at,password,email,phone,address) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&user.name, &user.created_at.to_string(), &user.password, &user.email, &user.phone, &user.address),
        )?;
        Ok(())
    }
    pub fn insert_random(&self)-> Result<()> {
        self.conn.execute(
            "INSERT INTO User (name,created_at,password,email,phone,address) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&self.fake_user.name, &self.fake_user.created_at.to_string(), &self.fake_user.password, &self.fake_user.email, &self.fake_user.phone, &self.fake_user.address),
        )?;
        Ok(())
    }
    pub fn get_user(&self, id: i32, name: String, password: String) -> Result<()> {
        self.conn.execute(
            "SELECT id, name ,password FROM User",
            (id, name, password),
        )?;
        for row in self.conn.prepare("SELECT id, name,created_at ,password, email, phone ,address FROM user")?.query_map([], |row| {
            Ok(User{
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get::<usize, NaiveDate>(2)?.into(),
                password: row.get(3)?,
                email: row.get(4)?,
                phone: row.get(5)?,
                address: row.get(6)?,
            })
        }).unwrap() {
            println!("Found User {:?}", row.unwrap());
        }
        Ok(())
    }
    
}