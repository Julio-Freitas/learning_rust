#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

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
fn main() {
    let routes: Vec<rocket::Route> = routes![
        index,
        get_user,
        create_user,
        delete_user,
        update_user,
        search_users
    ];

    rocket::ignite().mount("/", routes).launch();
}
