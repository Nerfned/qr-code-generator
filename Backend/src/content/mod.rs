mod email;
mod event;
mod paypal;
mod phonenumber;
mod sms;
mod url;
mod vcard;
mod whatsapp;

pub use self::{
    email::Email, event::Event, paypal::Paypal, phonenumber::Phonenumber, sms::Sms, url::Url,
    vcard::Vcard, whatsapp::Whatsapp,
};

pub trait Content {
    fn generate(&self) -> String;
}
