use crate::api::ResponseQR;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Responses {
    LoadQr {
        qrcodes: HashMap<i64, ResponseQR>,
    },
    Failed,
    SendData {
        username: String,
        email: String,
        icon: String,
    },
    SessionSaved {
        id: String,
        email: String,
        icon: String,
    },
    CreateDynQR{
        url: String,
    }
}
