use message_enum::{
    constants, messages::message_types::message_structs::pet_message::PetMessage,
    messaging_service::sending_client::SendingClient,
};
use rand::Rng;

use std::{thread, time};

fn main() {
    println!("Starting pet client");
    let mut rng = rand::thread_rng();
    loop {
        let message = PetMessage::from(rng.gen_range(65..91), rng.gen_range(0..2), rng.gen());
        let client = SendingClient::from(constants::LOCAL_HOST, constants::PET_PORT);
        println!("Sending {message} to server");
        client.send_message_server(message);
        let sleep_time = time::Duration::from_secs(2);
        thread::sleep(sleep_time);
    }
}
