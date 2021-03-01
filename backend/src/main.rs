#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

// Mongo Imports
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    sync::Client,
};

// Rocket Imports
use rocket::http::{Cookie, Cookies, SameSite};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    user_email: String,
    user_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct House {
    id: String,
    street_name: String,
    image: String,
    description: String,
    long_description: String,
    resident_type: String,
    price: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Team {
    id: String,
    name: String,
    image: String,
    description: String,
    member_type: String,
}

// GETS a specific house based on ID
#[get("/houses/<id>", format = "json")]
fn get_a_house(id: String, db: State<mongodb::sync::Database>) -> JsonValue {
    json!(get_a_house_from_mongo(&db, &id))
}

// Get all houses
#[get("/houses", format = "json")]
fn get_houses(db: State<mongodb::sync::Database>) -> JsonValue {
    json!(get_all_houses_from_mongo(&db))
}

// POST creates a new house entry
#[post("/house", format = "application/json", data = "<house>")]
fn post_house<'r>(
    house: Json<House>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    let user_and_password = cookies.get_private("access").unwrap().to_string();
    let user_and_password: Vec<&str> = user_and_password.split("=").collect();

    if check_user_exists(&db, user_and_password[1], user_and_password[2]) {
        if create_a_house_in_mongo(&db, &house) {
            "true"
        } else {
            "false"
        }
    } else {
        "Unauthroized access"
    }
}

// PATCH updates a house entry
#[patch("/house", format = "application/json", data = "<house>")]
fn patch_house<'r>(
    house: Json<House>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    let user_and_password = cookies.get_private("access").unwrap().to_string();
    let user_and_password: Vec<&str> = user_and_password.split("=").collect();

    if check_user_exists(&db, user_and_password[1], user_and_password[2]) {
        if update_a_house_in_mongo(&db, &house) {
            "true"
        } else {
            "false"
        }
    } else {
        "Unauthroized access"
    }
}

// DELETE updates a house entry
#[delete("/house", format = "application/json", data = "<house>")]
fn delete_house<'r>(
    house: Json<House>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    let user_and_password = cookies.get_private("access").unwrap().to_string();
    let user_and_password: Vec<&str> = user_and_password.split("=").collect();

    if check_user_exists(&db, user_and_password[1], user_and_password[2]) {
        if delete_a_house_in_mongo(&db, &house) {
            "true"
        } else {
            "false"
        }
    } else {
        "Unauthroized access"
    }
}

// GETS the team JSON file
#[get("/team", format = "json")]
fn get_team(db: State<mongodb::sync::Database>) -> JsonValue {
    json!(get_team_from_mongo(&db))
}

// POST creates a new house entry
#[post("/team", format = "application/json", data = "<team>")]
fn post_team<'r>(
    team: Json<Team>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    let user_and_password = cookies.get_private("access").unwrap().to_string();
    let user_and_password: Vec<&str> = user_and_password.split("=").collect();

    if check_user_exists(&db, user_and_password[1], user_and_password[2]) {
        if create_a_member_in_mongo(&db, &team) {
            "true"
        } else {
            "false"
        }
    } else {
        "Unauthroized access"
    }
}

// PATCH creates a new house entry
#[patch("/team", format = "application/json", data = "<team>")]
fn patch_team<'r>(
    team: Json<Team>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    let user_and_password = cookies.get_private("access").unwrap().to_string();
    let user_and_password: Vec<&str> = user_and_password.split("=").collect();

    if check_user_exists(&db, user_and_password[1], user_and_password[2]) {
        if update_a_member_in_mongo(&db, &team) {
            "true"
        } else {
            "false"
        }
    } else {
        "Unauthroized access"
    }
}

// DELETES a team entry
#[delete("/team", format = "application/json", data = "<team>")]
fn delete_team<'r>(
    team: Json<Team>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    let user_and_password = cookies.get_private("access").unwrap().to_string();
    let user_and_password: Vec<&str> = user_and_password.split("=").collect();

    if check_user_exists(&db, user_and_password[1], user_and_password[2]) {
        if delete_a_member_in_mongo(&db, &team) {
            "true"
        } else {
            "false"
        }
    } else {
        "Unauthroized access"
    }
}

// POST allows login to admin pages
#[post("/login", format = "application/json", data = "<user>")]
fn post_login<'r>(
    user: Json<User>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    if check_user_exists(&db, &user.user_email, &user.user_password) {
        let user_and_password = [user.user_email.as_str(), user.user_password.as_str()].join("=");

        let cookie = Cookie::build("access", user_and_password)
            .same_site(SameSite::Strict)
            .http_only(true)
            .finish();
        cookies.add_private(cookie);
        "true"
    } else {
        "false"
    }
}

// GET Deletes cookies for logout
#[get("/logout")]
fn get_logout(mut cookies: Cookies) {
    cookies.remove_private(Cookie::named("access"));
}

// GET checks if user is authorized to access admin pages
#[get("/auth")]
fn auth<'r>(mut cookies: Cookies, db: State<'r, mongodb::sync::Database>) -> &'r str {
    let user_and_password = cookies.get_private("access").unwrap().to_string();
    let user_and_password: Vec<&str> = user_and_password.split("=").collect();

    if check_user_exists(&db, user_and_password[1], user_and_password[2]) {
        "true"
    } else {
        "false"
    }
}

// Connect to the Mongo DB database
fn connect() -> mongodb::sync::Database {
    println!("Connecting to MongoDB");

    let client = Client::with_uri_str("mongodb+srv://shannonchow:samm2995@cluster0.htz6v.mongodb.net/proshop?authSource=admin&replicaSet=atlas-k9a80r-shard-0&readPreference=primary&appname=MongoDB%20Compass&ssl=true").unwrap();
    let db = client.database("realtor");

    println!("Connection Established");

    db
}

// Search for the user and see if user exists
fn check_user_exists(db: &mongodb::sync::Database, email: &str, password: &str) -> bool {
    let user_collection = db.collection("users");
    let filter = doc! { "username": email, "password": password };

    let cursor = user_collection.find(filter, None).unwrap();

    // Iterate over the results of the cursor.
    for result in cursor {
        let user_bson = result.unwrap();
        let user_email = user_bson.get("username").and_then(Bson::as_str).unwrap();
        let user_password = user_bson.get("password").and_then(Bson::as_str).unwrap();

        if (user_email == email) && (user_password == password) {
            return true;
        }
    }

    false
}

fn get_all_houses_from_mongo(db: &mongodb::sync::Database) -> Vec<House> {
    let house_collection = db.collection("houses");
    let cursor = house_collection.find(None, None).unwrap();

    let mut houses: Vec<House> = Vec::new();

    // Iterate over the results of the cursor.
    for result in cursor {
        let house_bson = result.unwrap();

        // Create a house struct
        let house = House {
            id: house_bson
                .get("_id")
                .and_then(Bson::as_object_id)
                .unwrap()
                .to_string(),
            street_name: house_bson
                .get("streetName")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
            image: house_bson
                .get("image")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
            description: house_bson
                .get("description")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
            long_description: house_bson
                .get("longDescription")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
            resident_type: house_bson
                .get("type")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
            price: house_bson.get("price").unwrap().to_string(),
        };

        houses.push(house);
    }

    houses
}

fn get_team_from_mongo(db: &mongodb::sync::Database) -> Vec<Team> {
    let team_collection = db.collection("team");
    let cursor = team_collection.find(None, None).unwrap();

    let mut team: Vec<Team> = Vec::new();

    // Iterate over the results of the cursor.
    for result in cursor {
        let team_bson = result.unwrap();

        // Create a house struct
        let member = Team {
            id: team_bson
                .get("_id")
                .and_then(Bson::as_object_id)
                .unwrap()
                .to_string(),
            name: team_bson
                .get("name")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
            image: team_bson
                .get("image")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
            description: team_bson
                .get("description")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
            member_type: team_bson
                .get("type")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
        };

        team.push(member);
    }

    team
}

fn create_a_member_in_mongo(db: &mongodb::sync::Database, team: &Team) -> bool {
    let team_collection = db.collection("team");
    let filter = doc! { "name" : &team.name };
    let cursor = team_collection.find(filter, None).unwrap();

    let team = doc! {
       "name": &team.name,
       "image": &team.image,
       "description": &team.description,
       "member_type": &team.member_type,
    };

    if cursor.count() > 0 {
        println!("Team member already exists!");
        return false;
    } else {
        let result = team_collection.insert_one(team, None);
        println!("{:?}", result);
        return true;
    };
}

fn update_a_member_in_mongo(db: &mongodb::sync::Database, team: &Team) -> bool {
    let team_collection = db.collection("team");
    let filter = doc! { "_id": ObjectId::with_string(&team.id).unwrap() };

    let team = doc! {
       "name": &team.name,
       "image": &team.image,
       "description": &team.description,
       "member_type": &team.member_type,
    };

    let result = team_collection.update_one(filter, team, None);
    println!("{:?}", result);
    true
}

fn delete_a_member_in_mongo(db: &mongodb::sync::Database, team: &Team) -> bool {
    let team_collection = db.collection("team");
    let filter = doc! { "_id": ObjectId::with_string(&team.id).unwrap() };

    let result = team_collection.delete_one(filter, None);
    println!("{:?}", result);
    true
}

fn get_a_house_from_mongo(db: &mongodb::sync::Database, id: &str) -> House {
    let house_collection = db.collection("houses");

    let filter = doc! { "_id": ObjectId::with_string(id).unwrap() };

    let cursor = house_collection.find(filter, None).unwrap();

    let mut house = House {
        id: String::new(),
        street_name: String::new(),
        image: String::new(),
        description: String::new(),
        long_description: String::new(),
        resident_type: String::new(),
        price: String::new(),
    };

    // Iterate over the results of the cursor.
    for result in cursor {
        let house_bson = result.unwrap();

        // Create a house struct
        house.id = house_bson
            .get("_id")
            .and_then(Bson::as_object_id)
            .unwrap()
            .to_string();
        house.street_name = house_bson
            .get("streetName")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string();
        house.image = house_bson
            .get("image")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string();
        house.description = house_bson
            .get("description")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string();
        house.long_description = house_bson
            .get("longDescription")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string();
        house.resident_type = house_bson
            .get("type")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string();
        house.price = house_bson.get("price").unwrap().to_string();
    }

    house
}

fn create_a_house_in_mongo(db: &mongodb::sync::Database, house: &House) -> bool {
    let house_collection = db.collection("houses");
    let filter = doc! { "streetName" : &house.street_name };
    let cursor = house_collection.find(filter, None).unwrap();

    let house = doc! {
        "streetName": &house.street_name,
        "image": &house.image,
        "description": &house.description,
        "longDescription": &house.long_description,
        "type": &house.resident_type,
        "price": &house.price,
    };

    if cursor.count() > 0 {
        println!("House already exists!");
        return false;
    } else {
        let result = house_collection.insert_one(house, None);
        println!("{:?}", result);
        return true;
    };
}

fn update_a_house_in_mongo(db: &mongodb::sync::Database, house: &House) -> bool {
    let house_collection = db.collection("houses");
    let filter = doc! { "_id": ObjectId::with_string(&house.id).unwrap() };

    let house = doc! {
        "streetName": &house.street_name,
        "image": &house.image,
        "description": &house.description,
        "longDescription": &house.long_description,
        "type": &house.resident_type,
        "price": &house.price,
    };

    let result = house_collection.update_one(filter, house, None);
    println!("{:?}", result);
    true
}

fn delete_a_house_in_mongo(db: &mongodb::sync::Database, house: &House) -> bool {
    let house_collection = db.collection("houses");
    let filter = doc! { "_id": ObjectId::with_string(&house.id).unwrap() };


    let result = house_collection.delete_one(filter, None);
    println!("{:?}", result);
    true
}

fn main() {
    let db = connect();

    rocket::ignite()
        .mount("/", routes![get_houses, get_a_house, get_team, auth])
        .mount(
            "/admin",
            routes![
                post_house,
                patch_house,
                delete_house,
                post_team,
                patch_team,
                delete_team,
                post_login,
                get_logout,
            ],
        )
        .manage(db)
        .launch();
}
