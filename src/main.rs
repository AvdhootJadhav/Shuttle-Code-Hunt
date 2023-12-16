use std::path::PathBuf;

use rocket::data::FromData;
use rocket::{
    get, post, routes,
    serde::{self, json::Json, Deserialize},
};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Reindeer {
    name: String,
    strength: i32,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Data(Vec<Reindeer>);

#[post("/4/strength", data = "<data>")]
fn calculate_strength(data: Json<Data>) -> String {
    let Json(data) = data;
    let Data(data) = data;

    let res = data
        .iter()
        .map(|data| data.strength)
        .sum::<i32>()
        .to_string();

    return res;
}

#[get("/1/<path..>")]
fn cube_the_bits(path: PathBuf) -> String {
    let mut calculated_value = -1;

    for num in path.iter() {
        let value = num.to_str().unwrap().parse::<i32>().unwrap();
        if calculated_value == -1 {
            calculated_value = value;
        } else {
            calculated_value ^= value;
        }
    }

    i32::pow(calculated_value, 3).to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, cube_the_bits, calculate_strength]);

    Ok(rocket.into())
}
