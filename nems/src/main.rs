/*
 * enable nightly function for using rocket
 */
#![feature(proc_macro_hygiene, decl_macro)]

mod httpd;

fn main() {
    httpd::start()
}