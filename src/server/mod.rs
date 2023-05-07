
use tide::Request;

use crate::{schemas::User, database::DataBase};

pub struct WebServer {
    db: DataBase
}
impl WebServer {
    pub async fn new()-> Self {
        let db = DataBase::new();
        db.create_table();
        let mut app = tide::new();
        app.at("/api/post_user").post(Self::post_user);
        app.at("/api/get_user").get(Self::get_user);
        app.listen("localhost:8080").await.unwrap();
        print!("Server is running on port 8080");
        Self {
            db
        }
    }
    //Get user endpoint
    pub async fn post_user(mut req: Request<()>) -> tide::Result {
        let db= DataBase::new();
        
        println!("INSERTED USER");
        let User { name, id, created_at, password, email, phone, address } = req.body_json().await.unwrap();
        db.insert_user(User { name, id, created_at, password, email, phone, address }).unwrap();
        Ok(format!("Inserted user").into())
    }
    pub async fn get_user(mut req: Request<()>) -> tide::Result {
        let db= DataBase::new();
        //db.get_user(id, name, password).unwrap();

        Ok(format!("Hello").into())
    }
}