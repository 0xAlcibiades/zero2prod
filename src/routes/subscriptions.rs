use actix_web::{post, HttpResponse, Responder, web};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}