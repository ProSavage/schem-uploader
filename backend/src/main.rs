use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::state::appstate::AppState;
use std::collections::HashMap;

mod state;

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Service: {}", &data.app_name) )
}

async fn auth(data: web::Data<AppState>) -> impl Responder {

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    println!("Starting backend on {}...", &addr);
    HttpServer::new(|| {
        App::new()
            .app_data(AppState {
                app_name: String::from("schem-uploader-backend"),
                tokens: HashMap::new()
            })
            .service(hello)
    })
        .bind(&addr)?
        .run()
        .await
}