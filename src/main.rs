#[macro_use] extern crate rocket;

use rocket::{tokio::sync::broadcast::{channel}};
use rocket::serde::{Serialize, Deserialize};

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}
// 1024 is the capacity of the channel, .0 is because we just want to store the first value in tupple, the sender
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/hello", routes![world])
}


