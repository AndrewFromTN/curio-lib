use futures::prelude::*;
use std::net::IpAddr;
use tokio::net::TcpStream;
use tokio::time::*;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

use crate::types::application::ConnectedApplicationType;
use crate::types::messages::ConnectMessage;
use crate::types::messages::NotificationMessage;
use crate::types::producer::ProducerTypes;

pub struct TestProducer {
    ip: IpAddr,
    port: u16,
}

impl TestProducer {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        TestProducer { ip, port }
    }

    pub async fn run(&self, message: NotificationMessage) {
        let my_app_type = ProducerTypes::API;
        let socket = TcpStream::connect((self.ip, self.port))
            .await
            .expect("Failed to connect to server.");

        let mut length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());
        let mut serialized_connect = tokio_serde::SymmetricallyFramed::new(
            &mut length_delimited,
            SymmetricalJson::<ConnectMessage>::default(),
        );

        serialized_connect
            .send(ConnectMessage {
                source_app: ConnectedApplicationType::Producer(my_app_type),
            })
            .await
            .expect("Failed to send.");

        delay_for(Duration::from_secs(4)).await;

        let mut serialized_notify = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<NotificationMessage>::default(),
        );

        serialized_notify
            .send(message)
            .await
            .expect("Failed to send.");
    }
}
