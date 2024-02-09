use actix_web::{
    web::{self, Json, Query},
    HttpResponse, Scope,
};
pub(crate) fn scope() -> Scope {
    web::scope("/5").default_service(web::post().to(handler))
}

#[derive(serde::Deserialize, Debug)]
struct QueryParams {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn handler(query_params: Query<QueryParams>, list: Json<Vec<String>>) -> HttpResponse {
    let offset = query_params.offset.unwrap_or_default();
    let limit = list
        .len()
        .min(offset + query_params.limit.unwrap_or(list.len()));
    if let Some(split) = query_params.split {
        let mut result = vec![];
        for chunk in list[offset..limit].chunks(split) {
            result.push(chunk);
        }
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::Ok().json(&list[offset..limit])
    }
}
