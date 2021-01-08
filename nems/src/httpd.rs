use rocket::{get, put, post, delete, head, patch, options, routes};
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;

use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use rocket_contrib::uuid::uuid_crate as uuid;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::sync::Mutex;

struct ShareData
{
  mutex: Mutex<HashMap<uuid::Uuid, &'static str>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PostData {
    name: String,
    message: String,
}

#[derive(Serialize)]
struct JsonResponse {
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
fn httpd_get() -> Json<JsonResponse> {
  Json(JsonResponse {
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

#[get("/people/<id>")]
fn people(id: Uuid, state: State<ShareData>) -> Result<String, String> {

  let mut lock = state.mutex.lock().expect("lock shared data");
  for (&contact, &number) in lock.iter() {
    println!("Calling {}: {}", contact, number);
  }

  lock.insert("7f205202-7ba1-4c39-b2fc-3e630722b39f".parse().unwrap(), "ccr");

  Ok(lock.get(&id)
    .map(|person| format!("We found: {}", person))
    .ok_or_else(|| format!("Person not found for UUID: {}", id))?)
}

pub fn httpd_register_handler ()
{

}

pub fn start(port: u32,
             www_root: String)
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

  let mut map = HashMap::new();
  map.insert("7f205202-7ba1-4c39-b2fc-3e630722bf9f".parse().unwrap(), "Lacy");
  map.insert("4da34121-bc7d-4fc1-aee6-bf8de0795333".parse().unwrap(), "Bob");
  map.insert("ad962969-4e3d-4de7-ac4a-2d86d6d10839".parse().unwrap(), "George");

  rocket::ignite()
        .manage(ShareData{mutex: Mutex::new(map)})
        .mount(
          &www_root.to_string(),
      routes![
          people,
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

pub fn stop()
{

}
