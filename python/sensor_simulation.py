import requests
import random
import time

# Simulate the sending of sensor data to the node.
def simulate_sensor_data():
    while True:
        data = {
            'temperature': random.uniform(20, 30),  # Random Temperature
            'energy': random.uniform(30, 100)       # Random Consumption of Energy
        }
        response = requests.post('http://localhost:3000/sensor-data', json = data)
        print(f"Sending data: {data}, Response: {response.text}")
        time.sleep(5)  # Simulate data every 5 seconds

if __name__ == '__main__':
    simulate_sensor_data()
