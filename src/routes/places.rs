use crate::model::Place;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde_json::json;


#[get("/places")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(
        vec![
            Place { id: 1, name: "pizza".to_string() },
            Place { id: 2, name: "kebab".to_string() },
        ]
    )
}

#[get("/places/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(
        Place { id: 1, name: "place".to_string() }
    )
}

#[post("/places")]
async fn create(place: web::Json<Place>) -> impl Responder {
    HttpResponse::Created().json(place.into_inner())
}

#[put("/places/{id}")]
async fn update(place: web::Json<Place>) -> impl Responder {
    HttpResponse::Ok().json(place.into_inner())
}

#[delete("/places/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Deleted"}))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
