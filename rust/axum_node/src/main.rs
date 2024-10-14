mod app;  // Importar la configuración de la aplicación
mod mqtt; // Importar módulo MQTT
mod models;
mod routes;
mod handlers;

#[tokio::main]
async fn main() {

    // Inicializar la aplicación Axum
    let app = app::create_app().await;

    // Iniciar el listener MQTT en segundo plano
    tokio::spawn(async move {
        mqtt::start_mqtt_listener().await;
    });

    // Configurar el servidor HTTP en el puerto 3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Servidor escuchando en {}", addr);

    // run our app with hyper, listening globally on port 3000
    axum::serve(listener, app).await.unwrap();
}