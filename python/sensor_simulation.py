import random
import time 
from datetime import datetime
from enum import Enum

# Enum to represent the state of the sensor
class Status(Enum):
    ACTIVE = 1
    INACTIVE = 2
    ERROR = 3

# Class that represents a temperature sensor
class TemperatureSensor:
    def __init__(self, sensor_id, status = Status.ACTIVE):
        self.sensor_id = sensor_id
        self.status = status
        self.temperature = None
    
    # Method to generate a new temperature reading
    def generate_temperature(self):
        if self.status == Status.ACTIVE:
            if self.temperature is None:
                # If it is the first reading, start with a random value
                self.temperature = round(random.uniform(15, 30), 2)
            else:
                # Modifies the previous value slightly
                variation = random.uniform(-0.5, 0.5)
                self.temperatura = round(min(max(self.temperatura + variation, 15), 30), 2)
        elif self.status == Status.INACTIVE:
            self.temperature = None
        elif self.status == Status.ERROR:
            self.temperature = 'ERROR'

    # Method to capture the temperature
    def capture_temperature(self):
        # Simulate the behavior of the sensor according to its status
        if self.status == Status.ACTIVE:
            # Simulate a random temperature between 15 and 30 degrees Celsius
            self.temperature = self.generate_temperature()
        elif self.status == Status.INACTIVE:
            self.temperature = None
        elif self.status == Status.ERROR:
            self.temperature = 'ERROR'

        # Return a tuple with the captured data
        return {
            'sensor_id': self.sensor_id,
            'status': self.status,
            'temperature': self.temperature,
            'timestamp': datetime.now().strftime('%Y-%m-%d %H:%M:%S')
        }

    def change_status(self, new_status):
        self.status = new_status

    def get_status(self):
        return self.status
    
    def change_status_with_probability(self):
        random_number = random.random()
        if random_number < 0.1:  # 10% of probability of error
            self.change_status(Status.ERROR)
        elif random_number < 0.2:  # 20% of probability of being inactive
            self.change_status(Status.INACTIVE)
        else:
            self.change_status(Status.ACTIVE)
        

# Simulate the operation of the sensor
def simulate_sensor():
    sensor = TemperatureSensor(sensor_id="T1")

    while True:
        # Capture data
        data = sensor.capture_temperature()
        print(data)

        # Change the status of the sensor
        sensor.change_status_with_probability()

        # Simulate a small delay between captures (5 seconds)
        time.sleep(5)

# Execute the simulation
simulate_sensor()

