use axum::{Router};
use crate::routes;

pub async fn create_app() -> Router {
    Router::new()
        .merge(routes::sensor_routes()) // Agrega las rutas relacionadas a sensores
}
