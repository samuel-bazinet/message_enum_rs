use message_enum::{constants, messages, thread_pool::ThreadPool};

use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    vec::Vec, process::abort,
};
fn main() {
    println!("Starting server");
    let message_storage = Arc::new(Mutex::new(Vec::new()));
    let socket = UdpSocket::bind(SocketAddr::from((
        constants::LOCAL_HOST,
        constants::SERVER_PORT,
    )))
    .unwrap();  

    let storage_accessor = message_storage.clone();

    _ = ctrlc::set_handler(move || {
        println!("Terminating server, printing stored messages");
        for i in storage_accessor.lock().unwrap().iter() {
            println!("{i}");
        }
        abort();
    });

    let pool = ThreadPool::new(3);

    loop {
        let mut buffer: [u8; 512] = [0; 512];
        let result = socket
            .recv_from(&mut buffer)
            .expect("Issue receiving a message");
        println!("Receive message of length {} from {}", result.0, result.1);
        let storage_accessor = message_storage.clone();
        pool.execute(move || handle_message(buffer, storage_accessor));
    }
}

fn handle_message(
    buffer: [u8; 512],
    message_storage: Arc<Mutex<Vec<messages::message_types::MessageTypes>>>,
) {
    let message_bytes = Vec::from_iter(buffer);
    let message = messages::message_types::MessageTypes::from_bytes(message_bytes);
    println!("Message has content: {message}");
    message_storage.lock().unwrap().push(message);
}
