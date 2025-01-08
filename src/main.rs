use rocket::{get, launch, routes};

#[get("/")]
async fn index() -> &'static str {
    "Hello Rocket"
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index])
}
