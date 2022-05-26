use serde::{Deserialize, Serialize};

use crate::types::application::ConnectedApplicationType;

#[derive(Serialize, Deserialize)]
pub struct ConnectMessage {
    pub source_app: ConnectedApplicationType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NotificationMessage {
    AuctionStatus(AuctionStatus),
    AuctionOutbid(AuctionOutbid),
    AuctionUpdate(AuctionUpdate),
    PasswordResetRequest(PasswordResetRequest),
    ValidateEmail(ValidateEmail),
    Init,
}

#[derive(Serialize, Deserialize)]
pub struct ConsumerSubscriptionMessage {
    pub message_subscriptions: Vec<NotificationMessage>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AuctionStatus {
    pub value: AuctionStatusValue,
    pub seller_email: String,
    pub buyer_email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum AuctionStatusValue {
    Bought(i64),
    Unsold,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuctionEndedMessage {
    pub status: AuctionStatusValue,
    pub seller: String,
    pub buyer: String,
    pub seller_email: String,
    pub buyer_email: String,
}

impl Default for AuctionEndedMessage {
    fn default() -> Self {
        AuctionEndedMessage {
            status: AuctionStatusValue::Unsold,
            seller: "".to_owned(),
            buyer: "".to_owned(),
            seller_email: "".to_owned(),
            buyer_email: "".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuctionOutbid {
    pub public_auction_id: String,
    pub price: f64,
    pub seconds_left: f64,
    pub outbidder: String,
    pub outbidee_email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReceived {
    pub sender: String,
    pub subject: String,
    pub receiver_email: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct AuctionUpdate {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct WatchedAuctionReminder {
    pub public_auction_id: String,
    pub auction_name: String,
    pub watcher_email: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct PasswordResetRequest {
    pub user_name: String,
    pub user_email: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct ValidateEmail {
    pub user_name: String,
    pub user_email: String,
    pub token: String,
}
