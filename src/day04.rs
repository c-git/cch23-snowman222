use actix_web::{web, HttpResponse, Scope};
pub(crate) fn scope() -> Scope {
    web::scope("/4")
        .route("/strength", web::post().to(task1_reindeer_cheer))
        .route(
            "/contest",
            web::post().to(task2_cursed_candy_eating_contest),
        )
}

#[tracing::instrument]
async fn task1_reindeer_cheer() -> String {
    todo!()
}

#[tracing::instrument]
async fn task2_cursed_candy_eating_contest() -> HttpResponse {
    todo!()
}
