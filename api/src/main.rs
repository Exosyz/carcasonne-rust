#[macro_use]
extern crate rocket;
mod controller;
mod routes;

use crate::routes::routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes())
}
