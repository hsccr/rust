/*
 * enable nightly function for using rocket
 */
#![feature(proc_macro_hygiene, decl_macro)]

mod httpd;

fn main() {
    httpd::httpd_register_handler();
    httpd::start(8000, "/".to_string());
    httpd::stop();
}