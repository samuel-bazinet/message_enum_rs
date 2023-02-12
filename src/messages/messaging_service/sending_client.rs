use super::super::message_types::message_structs::message_traits::serializable::Serializable;
use crate::constants;

use std::net::{SocketAddr, UdpSocket};

pub struct SendingClient {
    address: SocketAddr,
}

impl SendingClient {
    pub fn from(ip_address: [u8; 4], port: u16) -> Self {
        SendingClient {
            address: SocketAddr::from((ip_address, port)),
        }
    }

    pub fn send_message_server<T>(self, message: T)
    where
        T: Serializable,
    {
        let client_socket = UdpSocket::bind(self.address).unwrap();
        let message_bytes = message.serialize();
        let bytes_len = message_bytes.len();

        let sent_size = client_socket
            .send_to(
                message_bytes.as_slice().try_into().unwrap(),
                SocketAddr::from((constants::LOCAL_HOST, constants::SERVER_PORT)),
            )
            .unwrap();

        assert_eq!(bytes_len, sent_size);
    }
}
