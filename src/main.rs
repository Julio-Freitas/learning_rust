#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{ serde::json::Json};
use serde::Serialize;

#[macro_use]
extern crate rocket;
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    name: String,
    age: u32,
}

#[get("/exemple")]
fn todo() -> Json<Task> {
    Json(Task {
        age: 45,
        name: "Nome".to_string(),
    })
}

#[get("/")]
fn index() -> &'static str {
    "Hellor ROcket..."
}

// #[get("/hello/<name>")]
// fn hello(name: String) -> String {
//     format!("Hello, {}", name)
// }

// #[get("/search?<query>&<types>")]
// fn search(query: String, types: Option<String>) -> String {
//     match types {
//         Some(value) => format!("Searching for '{}', 'type:{}'", query, value),
//         None => format!("Searching for 'no type:{}'", query),
//     }
//}

#[get("/users/<id>")]
fn get_user(id: u32) -> String {
    format!("user ID:{}", id)
}

#[post("/users/<name>")]
fn create_user(name: String) {
    println!("Created user with name: {}", name);
}

#[delete("/users/<id>")]
fn delete_user(id: u32) {
    println!("Deleted user with name: {}", id);
}

#[put("/users?<id>&<name>")]
fn update_user(id: u32, name: String) {
    println!("updated user with name: {} para o nome {}", id, name);
}

#[get("/users?<query>&<page>")]
fn search_users(query: String, page: Option<u32>) -> String {
    match page {
        Some(page) => format!("Serching users with '{}' in the page {}", query, page),
        None => format!("Serching users with '{}' whithout the page.", query),
    }
}

#[catch(404)]
fn not_found() -> &'static str {
    "Hellor ROcket..."
}

#[launch]
fn rocket() -> _ {
    let routes: Vec<rocket::Route> = routes![
        index,
        get_user,
        create_user,
        delete_user,
        update_user,
        search_users,
        todo
    ];
    rocket::build()
        .mount("/", routes)
        .register("/", catchers![not_found])
}
