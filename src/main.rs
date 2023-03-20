#[macro_use]
extern crate rocket;

#[get("/")]
fn fetch() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch])
}
