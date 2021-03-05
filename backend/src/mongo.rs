use djangohashers::*;
use dotenv;

// Serde imports
use serde::{Deserialize, Serialize};

// Mongo Imports
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    sync::Client,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    name: String,
    email: String,
    phone: String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct House {
    id: String,
    street_name: String,
    image: String,
    description: String,
    long_description: String,
    resident_type: String,
    price: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    id: String,
    name: String,
    image: String,
    description: String,
    member_type: String,
}

// Connect to the Mongo DB database
pub fn connect() -> mongodb::sync::Database {
    println!("Connecting to MongoDB");

    let client = Client::with_uri_str(dotenv::var("MONGO_URL").unwrap().as_str()).unwrap();
    let db = client.database("realtor");

    println!("Connection Established");

    db
}

// Search for the user and see if user exists
pub fn check_user_exists(db: &mongodb::sync::Database, user: &User) -> bool {
    let user_collection = db.collection("users");


    let filter = doc! { "username": user.email.as_str() };

    let cursor = user_collection.find(filter, None).unwrap();

    if cursor.count() > 0 {
        return true;
    }

    false
}

pub fn get_all_houses_from_mongo(db: &mongodb::sync::Database) -> Vec<House> {
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

pub fn get_team_from_mongo(db: &mongodb::sync::Database) -> Vec<Team> {
    let team_collection = db.collection("team");
    let cursor = team_collection.find(None, None).unwrap();

    let mut team: Vec<Team> = Vec::new();

    // Iterate over the results of the cursor.
    for result in cursor {
        let team_bson = result.unwrap();

        // Create a team struct
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

pub fn get_a_member_from_mongo(db: &mongodb::sync::Database, id: &str) -> Team {
    let team_collection = db.collection("team");
    let filter = doc! { "_id": ObjectId::with_string(&id).unwrap() };
    let cursor = team_collection.find(filter, None).unwrap();

    let mut member = Team {
        id: String::new(),
        name: String::new(),
        image: String::new(),
        description: String::new(),
        member_type: String::new(),
    };

    // Iterate over the results of the cursor.
    for result in cursor {
        let team_bson = result.unwrap();

        // Create a house struct
        member = Team {
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
    }
    member
}

pub fn create_a_member_in_mongo(db: &mongodb::sync::Database, team: &Team) -> bool {
    let team_collection = db.collection("team");
    let filter = doc! { "name" : &team.name };
    let cursor = team_collection.find(filter, None).unwrap();

    let team = doc! {
       "name": &team.name,
       "image": &team.image,
       "description": &team.description,
       "type": &team.member_type,
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

pub fn update_a_member_in_mongo(db: &mongodb::sync::Database, team: &Team) -> bool {
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

pub fn delete_a_member_in_mongo(db: &mongodb::sync::Database, team: &Team) -> bool {
    let team_collection = db.collection("team");
    let filter = doc! { "_id": ObjectId::with_string(&team.id).unwrap() };

    let result = team_collection.delete_one(filter, None);
    println!("{:?}", result);
    true
}

pub fn get_a_house_from_mongo(db: &mongodb::sync::Database, id: &str) -> House {
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

pub fn create_a_house_in_mongo(db: &mongodb::sync::Database, house: &House) -> bool {
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

pub fn update_a_house_in_mongo(db: &mongodb::sync::Database, house: &House) -> bool {
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

pub fn delete_a_house_in_mongo(db: &mongodb::sync::Database, house: &House) -> bool {
    let house_collection = db.collection("houses");
    let filter = doc! { "_id": ObjectId::with_string(&house.id).unwrap() };

    let result = house_collection.delete_one(filter, None);
    println!("{:?}", result);
    true
}

pub fn create_a_contact_in_mongo(db: &mongodb::sync::Database, contact: &Contact) -> bool {
    let contacts_collection = db.collection("contacts");

    let doc = doc! {
        "name": contact.name.as_str(),
        "email": contact.email.as_str(),
        "phone": contact.phone.as_str(),
        "message": contact.message.as_str()
    };
    let _result = contacts_collection.insert_one(doc, None);
    true
}
