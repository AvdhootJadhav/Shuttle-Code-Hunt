use std::cmp::Ordering;
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

#[derive(Deserialize, Debug, serde::Serialize)]
#[serde(crate = "rocket::serde")]
struct Day4Response {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct DetailedReindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    c_an_d13s_3_ate_n_yes_t3rd_ay: i32,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct DetailedData(Vec<DetailedReindeer>);

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

#[post("/4/contest", data = "<input>")]
fn cursed_candy_contest(input: Json<DetailedData>) -> Json<Day4Response> {
    let Json(input) = input;
    let DetailedData(input) = input;

    let fastest = input
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap_or(Ordering::Equal))
        .unwrap();

    let tallest = input.iter().max_by_key(|data| data.height).unwrap();

    let magician = input.iter().max_by_key(|a| a.snow_magic_power).unwrap();

    let consumer = input
        .iter()
        .max_by_key(|a| a.c_an_d13s_3_ate_n_yes_t3rd_ay)
        .unwrap();

    let res = Day4Response {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    };

    return Json::from(res);
}

#[derive(Debug, serde::Serialize)]
#[serde(crate = "rocket::serde")]
struct Day6Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    elf: Option<i32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename(serialize = "elf on a shelf")
    )]
    elf_on_a_shelf: Option<i32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename(serialize = "shelf with no elf on it")
    )]
    shelf_with_no_elf_on_it: Option<i32>,
}

#[post("/6", data = "<data>")]
fn never_count_on_an_elf(data: String) -> Json<Day6Response> {
    let elf_data: Vec<_> = data.split_inclusive("elf").collect();
    let mut elf_count: Option<i32> = None;

    let elf_on_shelf: Vec<_> = data.split_inclusive("elf on a shelf").collect();
    let mut elf_shelf_count: Option<i32> = None;

    let shelf_count: i32 = data
        .split(" ")
        .filter(|a: &&str| a == &"shelf")
        .count()
        .try_into()
        .unwrap();

    for item in elf_data {
        let pos = item.find("elf");

        if pos.is_some() {
            if let Some(count) = elf_count {
                elf_count = Some(count + 1);
            } else {
                elf_count = Some(1);
            }
        }
    }

    for item in elf_on_shelf {
        let pos = item.find("elf on a shelf");

        if pos.is_some() {
            if let Some(count) = elf_shelf_count {
                elf_shelf_count = Some(count + 1);
            } else {
                elf_shelf_count = Some(1);
            }
        }
    }

    let mut elf_before_shelf: Option<i32> = None;

    if elf_shelf_count.is_none() {
        elf_before_shelf = None;
    } else {
        elf_before_shelf = Some((shelf_count - elf_shelf_count.unwrap()).try_into().unwrap());
    }

    let res = Day6Response {
        elf: elf_count,
        elf_on_a_shelf: elf_shelf_count,
        shelf_with_no_elf_on_it: elf_before_shelf,
    };

    return Json::from(res);
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
    let rocket = rocket::build().mount(
        "/",
        routes![
            index,
            cube_the_bits,
            calculate_strength,
            cursed_candy_contest,
            never_count_on_an_elf
        ],
    );

    Ok(rocket.into())
}
