use std::{thread, time::Duration};

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Delay server\nUsage: /wait/{milliseconds} - Wait for the selected milliseconds and finish the request"
}

#[get("/wait/{secs}")]
async fn delay(secs: web::Path<String>) -> impl Responder {
    match u64::from_str_radix(&secs, 10) {
        Ok(v) => {
            thread::sleep(Duration::from_millis(v));
            format!("Delayed for {} milliseconds!", secs)
        },
        Err(_) => {
            format!("Could not parse {}", secs)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(delay))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}