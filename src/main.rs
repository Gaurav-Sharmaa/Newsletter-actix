use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use env_logger::Env;

async fn greet(req: HttpRequest) -> impl Responder {
    // fixed spacing and simplified ownership
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // initialize logger (book recommends this so you see server/worker logs)
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
