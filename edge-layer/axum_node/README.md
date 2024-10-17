edge_node/
├── src/
│   ├── app.rs              # Configuración de la aplicación Axum
│   ├── main.rs             # Punto de entrada principal
│   ├── mqtt.rs             # Módulo para manejo de MQTT
│   ├── routes/
│   │   ├── mod.rs          # Archivo mod.rs que exporta las rutas
│   │   ├── sensor_data.rs  # Rutas y handlers relacionadas con datos de sensores
│   ├── handlers/
│   │   ├── mod.rs          # Archivo mod.rs que exporta los handlers
│   │   ├── sensor_handler.rs  # Lógica para manejar las solicitudes de datos de sensores
│   ├── models/
│   │   ├── mod.rs          # Archivo mod.rs que exporta los modelos de datos
│   │   ├── sensor_data.rs  # Definiciones de tipos y structs como SensorData
│   ├── mqtt/
│   │   ├── mod.rs          # Archivo mod.rs que organiza MQTT
│   │   ├── listener.rs     # Lógica para escuchar eventos MQTT
│   └── utils.rs            # Utilidades generales del proyecto
├── Cargo.toml              # Archivo de configuración del proyecto
