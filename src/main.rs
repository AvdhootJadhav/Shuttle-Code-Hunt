use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/1/<path..>")]
fn cube_the_bits(path: PathBuf) -> String {
    let mut calculated_value = -1;

    for num in path.iter() {
        let value = num.to_str().unwrap().parse::<i32>().unwrap();
        if calculated_value == -1 {
            calculated_value = value;
        }
        else {
            calculated_value ^= value;
        }
    }

    i32::pow(calculated_value, 3).to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
