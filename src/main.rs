#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
struct NumberResponse {
    number: u64,
    is_prime_number: bool,
    execution_time_in_micros: u128
}

#[get("/")]
fn index() -> &'static str {
    "This is my Rust prime number REST API"
}

#[get("/isPrime?<number>")]
fn get_is_prime(number: u64) -> Json<NumberResponse> {
    let now = SystemTime::now();

    Json(NumberResponse {
        number,
        is_prime_number: is_prime(number),
        execution_time_in_micros: now.elapsed().unwrap().as_micros(),
    })
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }

    true
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![index, get_is_prime])
        .launch()
        .await;
}
