use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SensorData {
    pub temperature: f64,
    pub energy: f64,
}
