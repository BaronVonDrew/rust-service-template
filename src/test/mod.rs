use crate::AppState;
use actix_web::{error::BlockingError, get, post, put, web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    name: String,
    body: String,
}
#[get("/")]
pub async fn test_get(_state: web::Data<AppState>) -> impl Responder {
    Ok::<HttpResponse, BlockingError>(HttpResponse::Ok().body("get"))
}

#[post("/")]
pub async fn test_post(_state: web::Data<AppState>, body: web::Json<NewPost>) -> impl Responder {
    Ok::<HttpResponse, BlockingError>(
        HttpResponse::Ok().body(format!("post {}, {}", body.name, body.body)),
    )
}

#[put("/{post_id}/publish")]
pub async fn test_put(_state: web::Data<AppState>, post_id: web::Path<i32>) -> impl Responder {
    Ok::<HttpResponse, BlockingError>(HttpResponse::Ok().body(format!("put {}", post_id)))
}

pub fn init(config: &mut web::ServiceConfig) {
    config
        .service(test_get)
        .service(test_post)
        .service(test_put);
}
