mod database;

use database::mongo_db::MongoDb;
use rocket::{get, launch, routes};
use rocket_db_pools::Database;

#[get("/")]
async fn index() -> &'static str {
    "Hello Rocket"
}

#[launch]
fn launch() -> _ {
    let _ = dotenv::dotenv();

    rocket::build()
        .attach(MongoDb::init())
        .mount("/", routes![index])
}
