pub(crate) mod error;
pub(crate) mod framed;
mod notification;
mod request;
mod response;
pub(crate) mod types;
// TODO Typed Result

use std::{convert::TryFrom, str::FromStr};

use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

pub(crate) use notification::Notification;
pub(crate) use request::Request;
pub(crate) use response::Response;
use types::Unknown;

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub(crate) enum Message {
    Request(Request),

    Notification(Notification),

    Response(Response),

    Unknown(Unknown),
}

impl From<Request> for Message {
    fn from(request: Request) -> Self {
        Message::Request(request)
    }
}

impl From<Notification> for Message {
    fn from(notification: Notification) -> Self {
        Message::Notification(notification)
    }
}

impl From<Response> for Message {
    fn from(response: Response) -> Self {
        Message::Response(response)
    }
}

impl From<Unknown> for Message {
    fn from(unknown: Unknown) -> Self {
        Message::Unknown(unknown)
    }
}

impl FromStr for Message {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl TryFrom<serde_json::Value> for Message {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

// We assume that all messages have `jsonrpc: "2.0"`.
impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct WithJsonRpc<'a, T: Serialize> {
            jsonrpc: &'static str,
            #[serde(flatten)]
            msg: &'a T,
        }

        match &self {
            Message::Request(request) => {
                let wrapped = WithJsonRpc {
                    jsonrpc: "2.0",
                    msg: &request,
                };
                wrapped.serialize(serializer)
            }

            Message::Notification(notification) => {
                let wrapped = WithJsonRpc {
                    jsonrpc: "2.0",
                    msg: &notification,
                };
                wrapped.serialize(serializer)
            }

            Message::Response(response) => {
                let wrapped = WithJsonRpc {
                    jsonrpc: "2.0",
                    msg: &response,
                };
                wrapped.serialize(serializer)
            }

            Message::Unknown(unknown) => {
                let wrapped = WithJsonRpc {
                    jsonrpc: "2.0",
                    msg: &unknown,
                };
                wrapped.serialize(serializer)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_request_from_str_or_value() {
        let v = json!({"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{}},"id":1});
        let from_str: Message = serde_json::from_str(&v.to_string()).unwrap();
        let from_value: Message = serde_json::from_value(v).unwrap();
        assert_eq!(from_str, from_value);
    }

    #[test]
    fn test_notification_from_str_or_value() {
        let v = json!({"jsonrpc":"2.0","method":"initialized","params":{}});
        let from_str: Message = serde_json::from_str(&v.to_string()).unwrap();
        let from_value: Message = serde_json::from_value(v).unwrap();
        assert_eq!(from_str, from_value);
    }

    #[test]
    fn test_response_from_str_or_value() {
        let v = json!({"jsonrpc":"2.0","result":{},"id":1});
        let from_str: Message = serde_json::from_str(&v.to_string()).unwrap();
        let from_value: Message = serde_json::from_value(v).unwrap();
        assert_eq!(from_str, from_value);
    }

    #[test]
    fn test_deserialize_unknown() {
        let v = json!({"jsonrpc":"2.0","method":"xinitialize","params":{"capabilities":{}},"id":1});
        let from_str: Message = serde_json::from_str(&v.to_string()).unwrap();
        let from_value: Message = serde_json::from_value(v).unwrap();
        assert_eq!(from_str, from_value);
    }
}