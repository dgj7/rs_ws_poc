mod model;
mod db;

#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use crate::model::Widget;

#[get("/widgets")]
fn widgets() -> Json<Vec<Widget>> {
    let widgets = vec!(
        Widget {id: 1, name: "harley davidson"},
        Widget {id: 2, name: "bicycle"},
    );
    Json(widgets)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/store", routes![widgets])
}
