use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::prelude::*;

async fn birthday_handler() -> HttpResponse {
    let birthday_month = "June";
    let birthday_day = 24;

    let current_date = Local::now();
    let current_month = current_date.format("%B").to_string();
    let current_day = current_date.day();

    if current_month == birthday_month && current_day == birthday_day {
        HttpResponse::Ok().body("Happy birthday, Rust developer! Enjoy your special day!")
    } else if current_month == birthday_month && current_day < birthday_day {
        let days_left = birthday_day - current_day;
        HttpResponse::Ok().body(format!(
            "Your birthday is in {} days. Looking forward to it!",
            days_left
        ))
    } else {
        HttpResponse::Ok().body("It's not your birthday yet, but it's coming soon!")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/birthday").to(birthday_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
