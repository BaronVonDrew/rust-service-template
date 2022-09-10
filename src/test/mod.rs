use actix_web::{error::BlockingError, get, post, put, web, HttpResponse, Responder};
use serde_derive::{Serialize,Deserialize};
use crate::{kafka, AppState};

#[derive(Serialize,Deserialize)]
struct NewPost { 
    body: String
}
#[get("/")]
pub async fn test_get(state: web::Data<AppState>) -> impl Responder {
    Ok::<HttpResponse, BlockingError>(HttpResponse::Ok().body("get"))
}

#[post("/")]
pub async fn test_post(state: web::Data<AppState>, body: web::Json<NewPost>) -> impl Responder {
    Ok::<HttpResponse, BlockingError>(HttpResponse::Ok().body("post"))
}

#[put("/{post_id}/publish")]
pub async fn test_put(state: web::Data<AppState>, post_id: web::Path<i32>) -> impl Responder {
    Ok::<HttpResponse, BlockingError>(HttpResponse::Ok().body("put"))
}

pub fn init(config: &mut web::ServiceConfig) {
    config
        .service(test_get)
        .service(test_post)
        .service(test_put);
}
