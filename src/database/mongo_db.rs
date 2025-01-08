use rocket_db_pools::{Database, mongodb::Client};

// Anotamos com #[derive(Database)], indicando que essa struct
// será gerenciada pelo rocket_db_pools.

#[derive(Database)]
#[database("mongodb")]
pub struct MongoDb(Client);
