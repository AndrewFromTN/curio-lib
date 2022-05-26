use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum ConsumerTypes {
    SMS,
    Email,
    WebsocketDispatch,
    Unknown,
}

impl From<String> for ConsumerTypes {
    fn from(string: String) -> Self {
        match &*string {
            "SMS" => Self::SMS,
            "Email" => Self::Email,
            "WebsocketDispatch" => Self::WebsocketDispatch,
            _ => Self::Unknown,
        }
    }
}

impl Into<String> for ConsumerTypes {
    fn into(self) -> String {
        match self {
            ConsumerTypes::SMS => String::from("SMS"),
            ConsumerTypes::Email => String::from("Email"),
            ConsumerTypes::WebsocketDispatch => String::from("WebsocketDispatch"),
            _ => String::from("Unknown"),
        }
    }
}

impl Into<String> for &ConsumerTypes {
    fn into(self) -> String {
        (*self).into()
    }
}
