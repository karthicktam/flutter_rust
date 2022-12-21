#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Res {
    pub hello: String
}


#[get("/hello/<name>")]
fn index(name: String) -> Json<Res> {
    Json(Res {hello: name})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}