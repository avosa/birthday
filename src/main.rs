use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BirthdayResponse {
    message: String,
}

#[async_graphql::Object]
impl BirthdayResponse {
    async fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Default)]
struct Query;

#[async_graphql::Object]
impl Query {
    async fn birthday(&self, _context: &Context<'_>) -> BirthdayResponse {
        let birthday_month = 6; // June (zero-based index)
        let birthday_day = 24;

        let current_date = Local::now();
        let current_month = current_date.month();
        let current_day = current_date.day();

        if current_month == birthday_month && current_day == birthday_day {
            BirthdayResponse {
                message: "Happy birthday, Rust developer! Enjoy your special day!".to_string(),
            }
        } else if current_month == birthday_month && current_day > birthday_day && current_day <= (birthday_day + 7) {
            let days_since_birthday = current_day - birthday_day;
            BirthdayResponse {
                message: format!(
                    "Happy belated birthday! It's been {} days since your birthday. Hope it was wonderful!",
                    days_since_birthday
                ),
            }
        } else if current_month == birthday_month && current_day > (birthday_day + 7) {
            BirthdayResponse {
                message: "It's not your birthday anymore, but I hope you had a great one!".to_string(),
            }
        } else if current_month == birthday_month && current_day < birthday_day {
            let days_left = birthday_day - current_day;
            BirthdayResponse {
                message: format!(
                    "Your birthday is in {} days. Looking forward to it!",
                    days_left
                ),
            }
        } else {
            BirthdayResponse {
                message: "It's not your birthday yet, but it's coming soon!".to_string(),
            }
        }
    }
}


async fn index(schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>) -> HttpResponse {
    let request = async_graphql::Request::new("{ birthday { message } }");

    let response = schema.execute(request).await;
    let json_response = serde_json::to_string(&response).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response)
}

async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(schema.clone())
            .service(web::resource("/").route(web::post().to(index)))
            .route("/", web::get().to(|| {
                HttpResponse::Found()
                    .header("Location", "/playground")
                    .finish()
            }))
            .route("/playground", web::get().to(playground))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
