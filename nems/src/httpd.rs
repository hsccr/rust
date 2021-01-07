use rocket::{get, put, post, delete, head, patch, options, routes};
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostData {
    pub name: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct State {
    state: String,
    message: String,
}

#[head("/")]
fn httpd_head() -> Custom<()> {
  Custom(Status::Ok, ())
}

#[delete("/")]
fn httpd_delete() -> Custom<()> {
  Custom(Status::Ok, ())
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
  format!("Hello, {} year old named {}!", age, name)
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
  name
}

#[get("/")]
fn httpd_get() -> Json<State> {
  Json(State {
    state: String::from("Ok"),
    message: String::from("call to get"),
  })
}

#[get("/custom")]
fn httpd_get_custom() -> Custom<()> {
  Custom(Status::Ok, ())
}

#[get("/str")]
fn httpd_get_str() -> &'static str {
  "call to str"
}

#[get("/string")]
fn httpd_get_string() -> String {
  "call to get string".to_string()
}

#[get("/json")]
fn httpd_get_json() -> Json<&'static str>
{
  Json("{
    'status': 'success',
    'message': 'call to get json'
  }")
}

#[put("/")]
fn httpd_put() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'call to put'
  }")
}

#[post("/", data = "<data>")]
fn httpd_post(data: Json<PostData>) ->Option<Json<PostData>>
{
  Some(Json(data.clone()))
}

#[patch("/")]
fn httpd_patch() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'call to path'
  }")
}

#[options("/")]
fn httpd_options() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'call to options'
  }")
}

pub fn start()
{
  println!("start ...");

  /*
    get - GET specific route
    put - PUT specific route
    post - POST specific route
    delete - DELETE specific route
    head - HEAD specific route
    options - OPTIONS specific route
    patch - PATCH specific route
  */

  rocket::ignite()
  .mount(
      "/",
      routes![
          hello,
          hi,
          httpd_head,
          httpd_delete,
          httpd_get,
          httpd_get_custom,
          httpd_get_str,
          httpd_get_string,
          httpd_get_json,
          httpd_put,
          httpd_post,
          httpd_patch,
          httpd_options,
      ],
  )
  .launch();
}

