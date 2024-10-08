use message_enum::{
    constants, messages::message_types::message_structs::human_message::HumanMessage,
    messaging_service::sending_client::SendingClient,
};
use rand::Rng;

use std::{thread, time};

fn main() {
    println!("Starting human client");
    let mut rng = rand::thread_rng();
    loop {
        let message = HumanMessage::new(
            rng.gen_range(65..91),
            rng.gen_range(65..91),
            rng.gen(),
            rng.gen(),
        );
        let client = SendingClient::from(constants::LOCAL_HOST, constants::HUMAN_PORT);
        println!("Sending {message} to server");
        client.send_message_server(message);
        let sleep_time = time::Duration::from_secs(1);
        thread::sleep(sleep_time);
    }
}
