use futures::prelude::*;
use std::net::{IpAddr, Shutdown};
use tokio::net::TcpStream;
use tokio::time::*;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::types::application::ConnectedApplicationType;
use crate::types::consumer::ConsumerTypes;
use crate::types::messages::ConnectMessage;
use crate::types::messages::ConsumerSubscriptionMessage;
use crate::types::messages::NotificationMessage;

pub struct TestConsumer {
    ip: IpAddr,
    port: u16,
    app_type: ConsumerTypes,
    socket: Option<TcpStream>,
    pub last_message_content: Option<NotificationMessage>,
}

impl TestConsumer {
    pub fn new(ip: IpAddr, port: u16, app_type: ConsumerTypes) -> Self {
        TestConsumer {
            ip,
            port,
            app_type,
            socket: None,
            last_message_content: None,
        }
    }

    pub async fn init_with_server(&mut self, subscribe_to: NotificationMessage) {
        self.socket = Some(
            TcpStream::connect((self.ip, self.port))
                .await
                .expect("Failed to connect to server."),
        );

        let mut my_socket = self.socket.as_mut().unwrap();
        let mut length_delimited_write =
            FramedWrite::new(&mut my_socket, LengthDelimitedCodec::new());
        let mut serialized = tokio_serde::SymmetricallyFramed::new(
            &mut length_delimited_write,
            SymmetricalJson::<ConnectMessage>::default(),
        );

        serialized
            .send(ConnectMessage {
                source_app: ConnectedApplicationType::Consumer(self.app_type),
            })
            .await
            .expect("Failed to send.");

        delay_for(Duration::from_secs(2)).await;

        let mut serialized_subscription = tokio_serde::SymmetricallyFramed::new(
            length_delimited_write,
            SymmetricalJson::<ConsumerSubscriptionMessage>::default(),
        );

        serialized_subscription
            .send(ConsumerSubscriptionMessage {
                message_subscriptions: vec![subscribe_to],
            })
            .await
            .expect("Failed to send.");
    }

    pub async fn read_messages(&mut self) {
        let mut my_socket = self.socket.as_mut().unwrap();
        let length_delimited_read = FramedRead::new(&mut my_socket, LengthDelimitedCodec::new());
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited_read,
            SymmetricalJson::<NotificationMessage>::default(),
        );

        while let Some(msg) = deserialized.next().await {
            match msg {
                Ok(msg) => {
                    self.last_message_content = Some(msg);
                }
                Err(_) => {
                    eprintln!("Consumer socket closed.");
                    break;
                }
            }
        }
    }

    pub fn kill_socket(&mut self) {
        self.socket
            .as_mut()
            .unwrap()
            .shutdown(Shutdown::Both)
            .unwrap();
    }
}
