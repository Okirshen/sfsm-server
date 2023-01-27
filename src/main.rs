#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, json::{Json}};
use rocket_db_pools::{sqlx, Database, Connection};

#[derive(Database)]
#[database("db")]
struct DB(sqlx::PgPool);

#[derive(Deserialize, Debug)]
struct Repo {
    #[serde(rename = "full_name")]
    name: String,
    #[serde(rename = "html_url")]
    url: String,
    description: String
}

#[derive(Deserialize, Debug)]
struct Release {
    #[serde(rename = "tag_name")]
    name: String,
    published_at: String,
    assets: Vec<String>
}

#[derive(Deserialize, Debug)]
struct Webhook {
    repository: Repo,
    release: Release,
}

#[post("/", format = "json", data = "<data>")]
fn index(data: Json<Webhook>, mut db: Connection<DB>) {
    println!("data: {:?}", data);
}

#[launch]
fn rocket() -> _ { 
    rocket::build()
    .attach(DB::init())
    .mount("/", routes![index])
}