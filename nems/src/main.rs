/*
 * enable nightly function for using rocket
 */
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;

mod httpd;

fn do_http_get(arg: i32) {
    println!("do_http_get {}", arg);
}

fn do_http_post(arg: i32) {
    println!("do_http_post {}", arg);
}

fn do_http_put(arg: i32) {
    println!("do_http_put {}", arg);
}

fn do_http_head(arg: i32) {
    println!("do_http_head {}", arg);
}

fn do_http_delete(arg: i32) {
    println!("do_http_delete {}", arg);
}

fn do_http_options(arg: i32) {
    println!("do_http_options {}", arg);
}

fn do_http_patch(arg: i32) {
    println!("do_http_patch {}", arg);
}

fn main() {
    httpd::httpd_register_handler("/".to_string(), do_http_get);
    httpd::httpd_register_handler("/".to_string(), do_http_post);
    httpd::httpd_register_handler("/".to_string(), do_http_put);
    httpd::httpd_register_handler("/".to_string(), do_http_head);
    httpd::httpd_register_handler("/".to_string(), do_http_delete);
    httpd::httpd_register_handler("/".to_string(), do_http_options);
    httpd::httpd_register_handler("/".to_string(), do_http_patch);
    httpd::start(8000, "/".to_string());
    httpd::stop();
}