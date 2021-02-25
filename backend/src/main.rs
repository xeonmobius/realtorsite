#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::{JsonValue};
use std::fs::File;


#[get("/houses/<id>", format = "json")]
fn get_a_house(id: usize) -> JsonValue {
    let file = File::open("../data/houses.json").unwrap();
    let json : serde_json::Value = serde_json::from_reader(file).expect("unable to read");

    json!(
        json.get("houses").unwrap().get(id)
    )
}

#[get("/houses", format = "json")]
fn get_houses() -> JsonValue {
    let file = File::open("../data/houses.json").unwrap();
    let json : serde_json::Value = serde_json::from_reader(file).expect("unable to read");

    json!(
        json
    )
}

#[get("/team", format = "json")]
fn get_team() -> JsonValue {
    let file = File::open("../data/team.json").unwrap();
    let json : serde_json::Value = serde_json::from_reader(file).expect("unable to read");

    json!(
        json
    )
}


fn main() {
    rocket::ignite().mount("/", routes![get_houses, get_a_house]).launch();
}