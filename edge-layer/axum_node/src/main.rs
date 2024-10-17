// mod app;  // Importar la configuración de la aplicación
// mod mqtt; // Importar módulo MQTT
// mod models;
// mod routes;
// mod handlers;

// #[tokio::main]
// async fn main() {

//     // Inicializar la aplicación Axum
//     let app = app::create_app().await;

//     // Iniciar el listener MQTT en segundo plano
//     tokio::spawn(async move {
//         mqtt::start_mqtt_listener().await;
//     });

//     // Configurar el servidor HTTP en el puerto 3000
//     let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
//     let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
//     println!("Servidor escuchando en {}", addr);

//     // run our app with hyper, listening globally on port 3000
//     axum::serve(listener, app).await.unwrap();
// }

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct SensorData {
    sensor_id: String,
    status: String,
    temperature: Option<f64>,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", post(receive_data));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler to receive the JSON and print it
async fn receive_data(Json(payload): Json<SensorData>) {
    // Print the received data
    println!("Received sensor data: {:?}", payload);
}