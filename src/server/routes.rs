use rocket_contrib::json::Json;
use crate::models::mdl_todo;

#[get("/")]
pub fn index() -> &'static str {
    "hello, World!"
}

#[get("/todos")]
pub fn todos() -> Json<Vec<mdl_todo::Todo>> {
  return Json(vec![mdl_todo::Todo {
    id: 1,
    title: "Read Rocket Tutorial".into(),
    description: "Read https://rocket.rs/guide/quickstart/".into(),
    done: false
  }]);
}

#[post("/todos", data = "<todo>")]
pub fn new_todo(todo: Json<mdl_todo::Todo>) -> String {
  return format!("Accepted post request! {:?}", todo.0)
}

#[get("/todos/<_todo_id>")]
pub fn todo_by_id(_todo_id: u32) -> String {
  let todo = mdl_todo::Todo {
    id: 1,
    title: "Read Rocket Tutorial".into(),
    description: "Read https://rocket.rs/guide/quickstart/".into(),
    done: false
  };
  return format!("{:?}", todo);
}
