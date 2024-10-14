use axum::Json;
use crate::models::sensor_data::SensorData;
//use pyo3::prelude::*;

// Proceso para manejar los datos del sensor
pub async fn process_sensor_data(
    Json(data): Json<SensorData>
) -> String {
    // let model = Python::with_gil(|py| {
    //     let model_module = PyModule::from_code(py, include_str!("model.py"), "", "").unwrap();
    //     model_module.get("LoadBalancingAI").unwrap().call1("model.joblib").unwrap()
    // });

    if should_redistribute(data.temperature, data.energy) {
        "Redistribuyendo carga a otro nodo...".to_string()
        // let response = client.post("http://localhost:3001/sensor-data")
        //     .json(&data)
        //     .send()
        //     .await;

        // match response {
        //     Ok(res) => format!("Redistribuyendo carga a otro nodo: {}", res.status()),
        //     Err(_) => "Error al redistribuir la carga".to_string(),
        // }
    } else {
        "Procesando datos localmente...".to_string()
    }
}

fn should_redistribute(temperature: f64, energy: f64) -> bool {
    // let features = vec![vec![temperature, energy]];
    // let prediction: Vec<i32> = model.call_method1("predict", (features,)).unwrap().extract().unwrap();
    // prediction[0] == 1
    temperature > 30.0
}
