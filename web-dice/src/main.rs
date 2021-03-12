use rand::{thread_rng, Rng};

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

fn dice_roll(sides: u8) -> Result<u8, &'static str> {
    if sides == 0 {
        return Err("Sides must be greater than zero");
    }
    Ok(thread_rng().gen_range(1..sides + 1))
}

#[get("/roll")]
async fn roll() -> impl Responder {
    match dice_roll(6) {
        Ok(result) => HttpResponse::Ok().body(result.to_string()),
        Err(msg) => HttpResponse::Ok().body(msg),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(roll))
        .bind("127.0.0.1:4000")?
        .run()
        .await
}
