use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum ProducerTypes {
    API,
    LiveWatch,
    Unknown,
}

impl From<String> for ProducerTypes {
    fn from(string: String) -> Self {
        match &*string {
            "API" => Self::API,
            "LiveWatch" => Self::LiveWatch,
            _ => Self::Unknown,
        }
    }
}

impl Into<String> for ProducerTypes {
    fn into(self) -> String {
        match self {
            ProducerTypes::API => String::from("API"),
            ProducerTypes::LiveWatch => String::from("Livewatch"),
            _ => String::from("Unknown"),
        }
    }
}

impl Into<String> for &ProducerTypes {
    fn into(self) -> String {
        (*self).into()
    }
}
