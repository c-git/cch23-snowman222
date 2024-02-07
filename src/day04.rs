use actix_web::{web, HttpResponse, Scope};
pub(crate) fn scope() -> Scope {
    web::scope("/4")
        .route("/strength", web::post().to(task1_reindeer_cheer))
        .route(
            "/contest",
            web::post().to(task2_cursed_candy_eating_contest),
        )
}

#[derive(serde::Deserialize, Debug)]
struct Reindeer {
    name: String,
    strength: u32,
}

#[tracing::instrument]
async fn task1_reindeer_cheer(reindeers: web::Json<Vec<Reindeer>>) -> String {
    reindeers
        .iter()
        .map(|x| x.strength)
        .sum::<u32>()
        .to_string()
}

#[tracing::instrument]
async fn task2_cursed_candy_eating_contest() -> HttpResponse {
    todo!()
}
