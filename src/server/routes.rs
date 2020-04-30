use rocket_contrib::json::Json;
use crate::models::Todo;

#[get("/")]
pub fn index() -> &'static str {
    "hello, World!"
}

#[get("/todos")]
pub fn todos() -> JSON<Vec<Todo>> {
  Json(vec![Todo {
    id: 1,
    title: "Read Rocket Tutorial".into(),
    description: "Read https://rocket.rs/guide/quickstart/".into(),
    done: false
  }])
}

#[post("/todos", data = "<todo>")]
pub fn newTodo(todo: Json<Todo>) -> String {
  format!("Accepted post request! {:?}", todo.0)
}

#[get("/todos/<todoid>")]
pub fn todoById(todoId: u32) -> String {
  let todo = Todo {
    id: 1,
    title: "Read Rocket Tutorial".into(),
    description: "Read https://rocket.rs/guide/quickstart/".into(),
    done: false
  }
  format!("{:?}", todo)
}