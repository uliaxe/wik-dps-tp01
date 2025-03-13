use actix_web::{get, HttpRequest,HttpResponse, Responder};

#[get("/ping")]
async fn ping(req:HttpRequest) -> impl Responder {
    let Headers = req
    .headers()
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
    .collect::<serde_json::Value>();

    HttpResponse::Ok().json(Headers)
}
