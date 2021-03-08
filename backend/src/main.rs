#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate djangohashers;

mod mongo;
use mongo::*;

// Rocket imports
use rocket::http::{Cookie, Cookies, SameSite, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

// Request Guard that checks if User is present in cookies and checks database
impl<'a, 'r> FromRequest<'a, 'r> for User {
    // Create an empty Error
    type Error = ();

    // Grab the request fn that is calling User
    fn from_request(request: &'a Request<'r>) -> Outcome<User, ()> {

        // Get the private user cookie
        let user_and_password: Option<Cookie> = request.cookies().get_private("access");

        // if cookie exists check if the user credential exist in database
        match user_and_password {
            None => {
                println!("No token.");
                rocket::Outcome::Forward(())
            }
            Some(user_and_password) => {
                let user_password = user_and_password.to_string();
                let user_password: Vec<&str> = user_password.split("=").collect();
                let db = request.guard::<State<mongodb::sync::Database>>().unwrap();
                let user = User {
                    email: user_password[1].to_string(),
                    password: user_password[2].to_string(),
                };

                // check if user exists in db
                match check_user_exists(&db, &user) {
                    false => {
                        println!("Invalid Token");
                        return Outcome::Forward(());
                    }
                    true => {
                        println!("Authroized Token");
                        return Outcome::Success(user);
                    }
                }
            }
        }
    }
}

// GETs all a house by id
#[get("/houses/<id>", format = "json")]
fn get_a_house(id: String, db: State<mongodb::sync::Database>) -> JsonValue {
    json!(get_a_house_from_mongo(&db, &id))
}

// GET all houses
#[get("/houses", format = "json")]
fn get_houses(db: State<mongodb::sync::Database>) -> JsonValue {
    json!(get_all_houses_from_mongo(&db))
}

// POST creates a new house entry
#[post("/house", format = "application/json", data = "<house>")]
fn post_house<'r>(
    house: Json<House>,
    _user: User,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    if create_a_house_in_mongo(&db, &house) {
        "true"
    } else {
        "false"
    }
}

// PATCH updates a house entry
#[patch("/house", format = "application/json", data = "<house>")]
fn patch_house<'r>(
    house: Json<House>,
    _user: User,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    if update_a_house_in_mongo(&db, &house) {
        "true"
    } else {
        "false"
    }
}

// DELETE updates a house entry
#[delete("/house", format = "application/json", data = "<house>")]
fn delete_house<'r>(
    house: Json<House>,
    _user: User,
    db: State<'r, mongodb::sync::Database>,
) -> &'r str {
    if delete_a_house_in_mongo(&db, &house) {
        "true"
    } else {
        "false"
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
fn post_team<'r>(team: Json<Team>, _user: User, db: State<'r, mongodb::sync::Database>) -> Status {
    if create_a_member_in_mongo(&db, &team) {
        Status::Ok
    } else {
        Status::BadRequest
    }
}

// PATCH creates a new team entry
#[patch("/team", format = "application/json", data = "<team>")]
fn patch_team<'r>(
    team: Json<Team>,
    _user: User,
    db: State<'r, mongodb::sync::Database>,
) -> Status {
    if update_a_member_in_mongo(&db, &team) {
        Status::Ok
    } else {
        Status::BadRequest
    }
}

// DELETES a team entry
#[delete("/team", format = "application/json", data = "<team>")]
fn delete_team<'r>(
    team: Json<Team>,
    _user: User,
    db: State<'r, mongodb::sync::Database>,
) -> Status {
    if delete_a_member_in_mongo(&db, &team) {
        Status::Ok
    } else {
        Status::BadRequest
    }
}

// GETS the blog JSON file
#[get("/blog")]
fn get_blogs(db: State<mongodb::sync::Database>) -> JsonValue {
    json!(get_all_blog_from_mongo(&db))
}

// GETS the blog JSON file
#[get("/blog/<id>")]
fn get_a_blog(id: String, db: State<mongodb::sync::Database>) -> JsonValue {
    json!(get_a_blog_from_mongo(&db, &id))
}

// POST creates a new team entry
#[post("/blog", format = "application/json", data = "<blog>")]
fn post_blog<'r>(blog: Json<Blog>, _user: User, db: State<'r, mongodb::sync::Database>) -> Status {
    if create_a_blog_in_mongo(&db, &blog) {
        Status::Ok
    } else {
        Status::BadRequest
    }
}

// PATCH creates a new team entry
#[patch("/blog", format = "application/json", data = "<blog>")]
fn patch_blog<'r>(
    blog: Json<Blog>,
    _user: User,
    db: State<'r, mongodb::sync::Database>,
) -> Status {
    if update_a_blog_in_mongo(&db, &blog) {
        Status::Ok
    } else {
        Status::BadRequest
    }
}

// DELETES a team entry
#[delete("/blog", format = "application/json", data = "<blog>")]
fn delete_blog<'r>(
    blog: Json<Blog>,
    _user: User,
    db: State<'r, mongodb::sync::Database>,
) -> Status {
    if delete_a_blog_in_mongo(&db, &blog) {
        Status::Ok
    } else {
        Status::BadRequest
    }
}

// POST allows login to admin pages
#[post("/login", format = "application/json", data = "<user>")]
fn post_login<'r>(
    user: Json<User>,
    mut cookies: Cookies,
    db: State<'r, mongodb::sync::Database>,
) -> Status {
    if check_user_exists(&db, &user) {
        let user_and_password = [user.email.as_str(), user.password.as_str()].join("=");

        // Create our auth cookie
        let cookie = Cookie::build("access", user_and_password)
            .same_site(SameSite::Strict)
            .http_only(true)
            .finish();

        cookies.add_private(cookie);
        Status::Ok
    } else {
        Status::NotFound
    }
}

// GET Deletes cookies for logout
#[get("/logout")]
fn get_logout(mut cookies: Cookies) {
    cookies.remove_private(Cookie::named("access"));
}

// GET checks if user is authorized to access admin pages
#[get("/auth")]
fn authorization<'r>(_user: User) -> Status {
    Status::Ok
}

#[post("/test", format = "application/json", data = "<team>")]
fn test(team: Json<Team>) -> String {
    println!("{:?}", team);

    "false".to_string()
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
                authorization,
                post_contactus,
                get_a_member,
                get_blogs,
                get_a_blog,
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
                get_logout,
                patch_blog,
                delete_blog,
                post_blog,
            ],
        )
        .manage(db)
        .launch();
}
