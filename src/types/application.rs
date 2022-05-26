use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::sync::broadcast;

use crate::types::consumer::ConsumerTypes;
use crate::types::messages::NotificationMessage;
use crate::types::producer::ProducerTypes;

#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum ConnectedApplicationType {
    Producer(ProducerTypes),
    Consumer(ConsumerTypes),
}

impl Into<String> for ConnectedApplicationType {
    fn into(self) -> String {
        match self {
            ConnectedApplicationType::Producer(producer_type) => {
                let app_type: String = producer_type.into();
                String::from("Producer: ") + app_type.as_ref()
            }
            ConnectedApplicationType::Consumer(consumer_type) => {
                let app_type: String = consumer_type.into();
                String::from("Consumer: ") + app_type.as_ref()
            }
        }
    }
}

impl Into<String> for &ConnectedApplicationType {
    fn into(self) -> String {
        (*self).into()
    }
}

pub struct ConnectedApplication {
    pub app_id: String,
    pub app_type: ConnectedApplicationType,
    pub socket: TcpStream,
    pub rx_channel: broadcast::Receiver<NotificationMessage>,
    pub tx_channel: broadcast::Sender<NotificationMessage>,
}
