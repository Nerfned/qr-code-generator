use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct QrJson {
    pub data: QrData,
    pub size: String,
    pub color: String,
    pub layout: Layout,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Base64 {
    pub logo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum QrData {
    #[serde(rename_all = "kebab-case")]
    Url { url: String },
    #[serde(rename_all = "kebab-case")]
    Vcard {
        name: String,
        surname: String,
        tnumber: String,
        mnumber: String,
        email: String,
        url: String,
        company: String,
        jobtitle: String,
        fax: String,
        address: String,
        city: String,
        postcode: String,
        country: String,
    },
    #[serde(rename_all = "kebab-case")]
    Paypal { user: String },
    #[serde(rename_all = "kebab-case")]
    Phonenumber { number: String },
    #[serde(rename_all = "kebab-case")]
    Sms { number: String, message: String },
    #[serde(rename_all = "kebab-case")]
    Email {
        email: String,
        subject: String,
        message: String,
    },
    #[serde(rename_all = "kebab-case")]
    Whatsapp { number: String, message: String },
    #[serde(rename_all = "kebab-case")]
    Event {
        name: String,
        place: String,
        start: isize,
        finish: isize,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Layout {
    Squares,        // "squares"
    Circles,        // "circles"
    RoundedSquares, // "rounded-squares"
    Rectangles,     // "rectangles"
}
