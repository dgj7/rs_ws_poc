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

#[get("/customer/shopping/products")]
fn customer_shopping_products() -> Json<Vec<String>> {
    Json(vec!("replace-me".to_string()))
}

#[get("/employee/manufacturing/pending")]
fn employee_manufacturing_pending() -> Json<Vec<String>> {
    Json(vec!("replace-me".to_string()))
}

#[get("/employee/manufacturing/open")]
fn employee_manufacturing_open() -> Json<Vec<String>> {
    Json(vec!("replace-me".to_string()))
}

#[get("/employee/manufacturing/completed")]
fn employee_manufacturing_completed() -> Json<Vec<String>> {
    Json(vec!("replace-me".to_string()))
}

#[get("/employee/management/reports/orders")]
fn employee_management_reports_orders() -> Json<Vec<String>> {
    Json(vec!("replace-me".to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        widgets,
        customer_shopping_products,
        employee_manufacturing_pending,
        employee_manufacturing_open,
        employee_manufacturing_completed,
        employee_management_reports_orders,
    ])
}
