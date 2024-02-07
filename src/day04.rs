use std::cmp::Ordering;

use actix_web::{web, HttpResponse, Scope};
use tracing::info;
pub(crate) fn scope() -> Scope {
    web::scope("/4")
        .route("/strength", web::post().to(task1_reindeer_cheer))
        .route(
            "/contest",
            web::post().to(task2_cursed_candy_eating_contest),
        )
}

#[derive(serde::Deserialize, Debug)]
struct Reindeer1 {
    strength: u32,
}

#[derive(serde::Deserialize, Debug)]
struct Reindeer2 {
    name: String,
    strength: u32,
    speed: f64,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

#[tracing::instrument]
async fn task1_reindeer_cheer(reindeers: web::Json<Vec<Reindeer1>>) -> String {
    reindeers
        .iter()
        .map(|x| x.strength)
        .sum::<u32>()
        .to_string()
}

#[tracing::instrument]
async fn task2_cursed_candy_eating_contest(reindeers: web::Json<Vec<Reindeer2>>) -> HttpResponse {
    info!("start");
    let fastest = reindeers
        .iter()
        .max_by(|x, y| {
            if x.speed < y.speed {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap();
    let tallest = reindeers.iter().max_by_key(|x| x.height).unwrap();
    let magician = reindeers.iter().max_by_key(|x| x.snow_magic_power).unwrap();
    let consumer = reindeers
        .iter()
        .max_by_key(|x| x.candies_eaten_yesterday)
        .unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "fastest": format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name),
        "tallest": format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width),
        "magician": format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power),
        "consumer": format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food),
    }
        ))
}
