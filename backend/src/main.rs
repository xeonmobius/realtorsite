#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate djangohashers;

mod mongo;
use mongo::*;

// Rocket imports
use rocket::http::{Cookie, Cookies, SameSite};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

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
    let user = User {
        email: user_and_password[1].to_string(),
        password: user_and_password[2].to_string(),
    };
    if check_user_exists(&db, &user) {
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
    let user = User {
        email: user_and_password[1].to_string(),
        password: user_and_password[2].to_string(),
    };
    if check_user_exists(&db, &user) {
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
    let user = User {
        email: user_and_password[1].to_string(),
        password: user_and_password[2].to_string(),
    };

    if check_user_exists(&db, &user) {
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

// GETS the team JSON file
#[get("/team/<id>")]
fn get_a_member(id: String, db: State<mongodb::sync::Database>) -> JsonValue {
    json!(get_a_member_from_mongo(&db, &id))
}

// POST creates a new team entry
#[post("/team", format = "application/json", data = "<team>")]
fn post_team<'r>(
    team: Json<Team>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    let user_and_password = cookies.get_private("access").unwrap().to_string();
    let user_and_password: Vec<&str> = user_and_password.split("=").collect();
    let user = User {
        email: user_and_password[1].to_string(),
        password: user_and_password[2].to_string(),
    };

    if check_user_exists(&db, &user) {
        if create_a_member_in_mongo(&db, &team) {
            "true"
        } else {
            "false"
        }
    } else {
        "Unauthroized access"
    }
}

// PATCH creates a new team entry
#[patch("/team", format = "application/json", data = "<team>")]
fn patch_team<'r>(
    team: Json<Team>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    let user_and_password = cookies.get_private("access").unwrap().to_string();
    let user_and_password: Vec<&str> = user_and_password.split("=").collect();
    let user = User {
        email: user_and_password[1].to_string(),
        password: user_and_password[2].to_string(),
    };

    if check_user_exists(&db, &user) {
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

    let user = User {
        email: user_and_password[1].to_string(),
        password: user_and_password[2].to_string(),
    };

    if check_user_exists(&db, &user) {
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
    if check_user_exists(&db, &user) {
        let user_and_password = [user.email.as_str(), user.password.as_str()].join("=");

        // Create our auth cookie
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

    let user = User {
        email: user_and_password[1].to_string(),
        password: user_and_password[2].to_string(),
    };

    if check_user_exists(&db, &user) {
        "true"
    } else {
        "false"
    }
}

// POST that saves user contact info
#[post("/contactus", format = "application/json", data = "<contact>")]
fn post_contactus<'r>(contact: Json<Contact>, db: State<'r, mongodb::sync::Database>) -> &'r str {
    create_a_contact_in_mongo(&db, &contact);
    "true"
}

fn main() {
    let db = connect();

    rocket::ignite()
        .mount(
            "/",
            routes![
                get_houses,
                get_a_house,
                get_team,
                auth,
                post_contactus,
                get_a_member
            ],
        )
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
                get_logout
            ],
        )
        .manage(db)
        .launch();
}
