use futures::prelude::*;
use futures::StreamExt;
use std::vec::Vec;
use tokio::net::TcpStream;
use tokio::sync::watch::Sender;
use tokio::time::*;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::types::application::ConnectedApplicationType;
use crate::types::consumer::ConsumerTypes;
use crate::types::messages::ConnectMessage;
use crate::types::messages::ConsumerSubscriptionMessage;
use crate::types::messages::NotificationMessage;

pub struct ConsumerService {
    pub server_addr: String,
    pub consumer_service_addr: String,
    pub service_type: ConsumerTypes,
    pub subscriptions: Vec<NotificationMessage>,
    pub tx: Sender<NotificationMessage>,
}

impl ConsumerService {
    pub fn new(
        server_addr: String,
        consumer_service_addr: String,
        service_type: ConsumerTypes,
        subscriptions: Vec<NotificationMessage>,
        tx: Sender<NotificationMessage>,
    ) -> Self {
        ConsumerService {
            server_addr,
            consumer_service_addr,
            service_type,
            subscriptions,
            tx,
        }
    }

    pub async fn establish_connection(&mut self) {
        let mut socket: TcpStream;
        loop {
            match TcpStream::connect(&self.server_addr).await {
                Ok(s) => {
                    socket = s;
                }
                Err(_e) => {
                    eprintln!("Failed to connect to server.");
                    delay_for(Duration::from_secs(10)).await;
                    continue;
                }
            }

            let mut length_delimited_write =
                FramedWrite::new(&mut socket, LengthDelimitedCodec::new());
            let mut serialized = tokio_serde::SymmetricallyFramed::new(
                &mut length_delimited_write,
                SymmetricalJson::<ConnectMessage>::default(),
            );

            serialized
                .send(ConnectMessage {
                    source_app: ConnectedApplicationType::Consumer(self.service_type),
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
                    message_subscriptions: self.subscriptions.clone(),
                })
                .await
                .expect("Failed to send.");

            delay_for(Duration::from_secs(2)).await;

            let length_delimited_read = FramedRead::new(&mut socket, LengthDelimitedCodec::new());
            let mut deserialized = tokio_serde::SymmetricallyFramed::new(
                length_delimited_read,
                SymmetricalJson::<NotificationMessage>::default(),
            );

            while let Some(msg) = deserialized.next().await {
                match msg {
                    Ok(msg) => {
                        dbg!("Message: {:?}", &msg);
                        if let Err(e) = self.tx.broadcast(msg) {
                            eprintln!("Error sending websocket message: {}", e);
                        }
                    }
                    Err(_) => {
                        eprintln!("Consumer socket closed.");
                        continue;
                    }
                }
            }
        }
    }
}
