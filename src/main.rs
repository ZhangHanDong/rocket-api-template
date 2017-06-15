#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
extern crate regex;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


mod api;

#[cfg(test)] mod tests;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![api::hello_world::kiss])
        .catch(errors![api::errors::not_found, api::errors::internal_error])
}

fn main() {
    rocket().launch();
}
