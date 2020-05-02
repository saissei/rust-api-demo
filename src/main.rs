#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod models;
mod server;

use server::routes;
pub use self::models::mdl_todo;

fn main() {
  rocket::ignite()
    .mount("/", routes![routes::index, routes::todos, routes::new_todo, routes::todo_by_id])
    .launch();
}
