#[macro_use] extern crate rocket;

use rocket::Route;

#[get("/hello")]
fn hello() -> String {
    "Hello !".into()
}

pub fn routes() -> Vec<Route> {
    routes![hello]
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes())
}