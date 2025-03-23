use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware::Logger};
use serde::Serialize; // Importamos el trait Serialize para poder serializar el struct

// Estructura que representa la respuesta en formato JSON
#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

// Handler para el endpoint /health
async fn health() -> impl Responder {
    // Creamos una instancia de HealthResponse con el valor "UP"
    let response = HealthResponse {
        status: String::from("UP"),
    };

    // Respondemos con un JSON serializado
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar el logger (env_logger maneja la variable de entorno por sí mismo)
    env_logger::init();

    // Iniciar el servidor HTTP en la dirección localhost:8080
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Activamos el middleware de logging
            .route("/health", web::get().to(health))  // Definir el endpoint /health
    })
    .bind("127.0.0.1:8080")?  // Puedes cambiar la IP o el puerto si lo necesitas
    .run()
    .await
}
