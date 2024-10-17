use rumqttc::{AsyncClient, LastWill, MqttOptions, QoS};
use tokio::task;
use tokio::time::{sleep, Duration};

/*
 * This is the main function of the program. In this function, we initialize an MQTT client,
 * set connection options and last will message. Then we create a client and a connection,
 * and call the publish function in a new thread. Next, we use connection.iter()
 * method to iterate through the notifications in the connection and handle each notification.
 */
pub async fn start_mqtt_listener() {
    // Initialize the logger
    pretty_env_logger::init();

    // Set MQTT connection options and last will message
    let mut mqttoptions = MqttOptions::new("rust_axum", "broker.emqx.io", 1883);
    // topic = "hello/world", message = "good bye", QoS = AtMostOnce, retain = false
    // test-1 is the client ID, broker.emqx.io is the broker address, and 1883 is the port number
    let will = LastWill::new("rust/mqtt_example", "good bye", QoS::AtMostOnce, false);
    mqttoptions
        .set_keep_alive(Duration::from_secs(5))
        .set_last_will(will);
    
    // Create MQTT client and connection, and call the publish function in a new thread
    let (client, mut connection) = AsyncClient::new(mqttoptions, 10);
    
    // Ejecuta la función de publicación en un nuevo task asíncrono
    task::spawn(async move {
        publish(client).await;
    });

     // Itera sobre las notificaciones en la conexión
    //  while let Ok(notification) = connection.poll().await {
    //     println!("Received = {:?}", notification);
    // }
    // Iterate over notifications and handle received messages
    while let Ok(notification) = connection.poll().await {
        match notification {
            // Check for incoming publish notifications
            rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) => {
                // Print the received message
                println!(
                    "Received message: Topic: {}, Payload: {:?}",
                    publish.topic,
                    String::from_utf8_lossy(&publish.payload)
                );
            }
            // Print any other notifications
            _ => {
                println!("Received = {:?}", notification);
            }
        }
    }

    println!("¡Terminó la conexión!");
}

/*
 * This is a helper function for publishing MQTT messages. In this function, we first sleep
 * for one second, then subscribe to a topic. Then we loop and send ten messages with lengths
 * ranging from 0 to 9, with each message's QoS being at least once.
 */
async fn publish(client: AsyncClient) {
    // Wait for one second before subscribing to a topic
    sleep(Duration::from_secs(1)).await;
    client.subscribe("rust/mqtt_example", QoS::AtMostOnce).await.unwrap();
    
    // Send ten messages with lengths ranging from 0 to 9, each message's QoS being at least once
    let payload = "Hello world!".as_bytes().to_vec();
    let topic = format!("rust/mqtt_example");
    let qos = QoS::AtLeastOnce;

    client.publish(topic, qos, true, payload).await.unwrap();
    sleep(Duration::from_secs(1)).await; // Add a delay between publishes

    sleep(Duration::from_secs(1)).await;
}
