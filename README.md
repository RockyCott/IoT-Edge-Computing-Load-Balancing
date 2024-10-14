# IoT Edge Computing with Load Balancing using Rust and Python

This project demonstrates a simple Internet of Things (IoT) architecture using edge computing to distribute the processing load between multiple nodes based on conditions determined by a basic AI model. The nodes are built with **Rust** (using the **Axum** framework), while the IoT devices are simulated in **Python**. A machine learning model is used to decide whether to process data locally or redistribute it to another node.

## Features

- **IoT Sensor Simulation**: Simulates IoT devices that send data to an edge node for processing.
- **Edge Computing**: Each edge node receives and processes sensor data or redistributes the load to another node.
- **Load Balancing with AI**: A simple AI model is used to decide whether the load should be processed locally or sent to another node.
- **Multi-node Setup**: Multiple Rust nodes running in Docker containers, with inter-node communication enabled.

## Technologies

- **Rust**: Backend services are implemented with the Axum web framework.
- **Axum**: Web framework in Rust for handling HTTP requests.
- **Python**: Simulates IoT devices that generate and send sensor data.
- **Machine Learning**: A basic model (saved in `joblib` format) is used to make decisions on load distribution.
- **Docker**: Used to containerize and manage multiple nodes.

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

## Code Structure
- rust/axum_node/: Contains the Rust code for the edge nodes.
- python/sensor_simulation.py: Python script to simulate IoT sensor data.
- docker-compose.yml: Docker configuration to run multiple nodes.

## Future Improvements
- Advanced AI Models: Replace the basic model with more sophisticated algorithms to make more complex load-balancing decisions.

- Edge Caching: Implement edge caching mechanisms to reduce latency.
Security: Add authentication and authorization mechanisms to secure inter-node communication.

## Contributing
If you'd like to contribute, feel free to submit a pull request or open an issue to discuss improvements.