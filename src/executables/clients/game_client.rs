use message_enum::{
    constants, messages::message_types::message_structs::game_message::GameMessage,
    messaging_service::sending_client::SendingClient,
};
use rand::Rng;

use std::{thread, time};

fn main() {
    println!("Starting pet client");
    let mut rng = rand::thread_rng();
    loop {
        let message = GameMessage::from(
            rng.gen(),
            rng.gen(),
            rng.gen(),
        );
        let client = SendingClient::from(constants::LOCAL_HOST, constants::HUMAN_PORT);
        println!("Sending {message} to server");
        client.send_message_server(message);
        let sleep_time = time::Duration::from_secs(3);
        thread::sleep(sleep_time);
    }
}
