# Funcionamiento de simular un sensor

- **TemperatureSensor:** Es la clase que representa el sensor. Tiene un estado (activo, inactivo, error), y una función capture_temperature que simula la captura de datos.

  - Cuando el sensor está activo, genera una temperatura aleatoria entre 15°C y 30°C.
  - Si está inactivo, no captura datos.
  - Si está en estado de error, devuelve un mensaje de error.

- **Simulation:** La función simulate_sensor ejecuta la captura de datos del sensor cada 5 segundos, simulando un ciclo de funcionamiento típico.

- **Sensor status:** Se pueden cambiar mediante la función cambiar_estado, lo que te permitirá simular situaciones donde el sensor esté inactivo o tenga fallos.