# SixEyes Edge

[![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=fff)](https://www.python.org/)
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A scalable framework for integrating IoT devices with edge computing nodes, inspired by precision and control.

This project aims to build a scalable framework that integrates **IoT (Internet of Things)**, **Edge Computing**, and potential **AI** components. The framework provides a modular architecture to simulate sensors, process data on edge devices, and transmit data to a central server.

## Project Structure

- **Sensor Simulation**: Simulates IoT devices (starting with temperature sensor) that send data to an edge node for processing. Built in Python.
- **Edge Layer**: Holds the Rust-based Axum server which acts as an edge device receiving and processing sensor data or redistributes the load.
- **Api Design Docs**: Documentation detailing API design standards, conventions, and available endpoints.
- **Tests**: Unit and integration tests for both the sensors and the edge layer.
- **Docs**: General documentation for the architecture, design decisions, and roadmap.
- **Multi-node Setup**: Multiple Rust nodes running in Docker containers, with inter-node communication enabled.

## Getting Started

### Prerequisites

- **Python** Simulates IoT devices that generate and send sensor data.
- **Rust** for edge layer development.
- **Cargo**: Rust’s package manager to build and run the server.

### Setup

1. **Clone the repository**:
    ```bash
    git clone https://github.com/RockyCott/SixEyes-Edge.git
    cd iot-edge-framework
    ```

2. **Install Python dependencies** (for sensor simulations):

In the `python/` folder, there is a `requirements.txt` file. 
    ```bash
    cd sensor-simulations/temperature_sensor
    pip install -r requirements.txt
    ```

3. **Run the temperature sensor simulation**:
    ```bash
    python sensor_simulation.py
    ```

4. **Run the Rust edge server** (in another terminal):
    ```bash
    cd edge-layer/axum_server
    cargo run
    ```

5. The sensor will send HTTP POST requests to the Axum server. Check the console for the received data.

### Sensor Payload Structure

```json
{
    "sensor_id": "T1",
    "temperature": 22.5,
    "status": "active",
    "timestamp": "2024-10-16T12:45:00Z"
}
```

## Prerequisites

Before starting, ensure you have the following installed:

- **Python 3.8+**
- **Rust** (if you want to run without Docker)
- **Docker** (to run containers)
- **Arch Linux users**: See the specific instructions below for handling Python dependencies.

In the `python/` folder, there is a `requirements.txt` file. 

#### On non-Arch Linux systems:

You can install the dependencies by running:

```bash
pip install -r requirements.txt
```

#### On Arch Linux:

Arch Linux has a different way of managing Python dependencies system-wide. To avoid issues with externally-managed-environment errors, use a virtual environment or pipx. Here are the recommended methods:

Create a Python virtual environment:
```bash
cd python
python -m venv venv           # Create a virtual environment
source venv/bin/activate       # Activate the virtual environment
pip install -r requirements.txt  # Install the dependencies
```

2. Using pipx for application isolation:

If you want to isolate Python applications:
```bash
sudo pacman -S python-pipx    # Install pipx
pipx install some_package      # Replace 'some_package' with the package you need
```

Now you can run the Python simulation script without system-wide conflicts.

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/
cd IoT-Edge-Computing-Load-Balancing
```

### 2. Build and run the services with Docker

To run the entire setup using Docker:
```bash
docker-compose up --build
```

This will spin up multiple Rust nodes and simulate the edge computing environment.

### 3. Simulate IoT sensor data

In a separate terminal, run the Python simulation script to start sending sensor data:

If using a virtual environment (recommended for Arch Linux users):
```bash
cd python
source venv/bin/activate       # Activate the virtual environment
python sensor_simulation.py
```

For non-Arch Linux users:
```bash
cd python
python sensor_simulation.py
```

This script will send sensor data to the main node, which will either process it locally or redistribute it to another node based on the AI model's decision.

### 4. Verify the load distribution

Check the logs of the different nodes (node1, node2, node3) to see where the data is being processed:
```bash
docker logs node1
docker logs node2
docker logs node3
```

### 5. Customizing the AI model

The AI model used for load balancing is a basic classification model loaded from the ai_model.pkl file. You can replace this model with your own machine learning model by training it on relevant data and saving it in joblib format.

## Why Consider it an Architectural Framework?

1. **Scalable Architecture**: It provides a layered design that allows easy expansion of devices, data, and edge nodes, ensuring the system can grow as needed without sacrificing performance.
2. **Standardization**: By implementing strict naming conventions, API standards, and modular components, the framework promotes consistency and best practices across all components.
3. **Edge Focused**: Unlike traditional IoT solutions, this framework emphasizes the importance of processing data at the edge, closer to the source, minimizing latency and reducing load on central servers.
4. **Customizable Components**: The framework is highly modular, allowing for integration of new sensors, edge devices, and cloud components without disrupting the core system.

Whether you're building a small IoT project or a complex network of edge devices, the SixEyes Edge Framework offers a standardized approach to design, development, and deployment.

## Name Justification

The framework is named **SixEyes Edge** as an homage to **Gojo Satoru's *Six Eyes* technique** from ***Jujutsu Kaisen***, which grants him unparalleled **perception and control**. In a similar fashion, the framework enables the seamless monitoring and management of data generated by IoT devices at the edge, ensuring optimized data processing and scalability. Just as Gojo's *Six Eyes* allows him to perceive everything with precision, this framework is designed to handle complex IoT environments with clarity and efficiency, allowing developers to manage networks of devices and edge computing nodes in a flexible and scalable manner.

## Contributing

If you'd like to contribute, feel free to submit a pull request or open an issue to discuss improvements. Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Thank you for using and reviewing about SixEyes Edge. ❤️
