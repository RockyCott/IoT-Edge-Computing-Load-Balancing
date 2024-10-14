use crate::handlers::sensor_handler;
use axum::{routing::post, Router};

pub fn sensor_routes() -> Router {
    Router::new().route(
        "/temperature-data",
        post(sensor_handler::process_sensor_data),
    )
}
