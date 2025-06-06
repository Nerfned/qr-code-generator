use crate::{
    content::{Content, Email, Event, Paypal, Phonenumber, Sms, Url, Vcard, Whatsapp},
    qr_json::QrData,
};

pub fn qrdata(body: QrData) -> Option<String> {
    let mut _data = None;
    match body {
        QrData::Url { url } => {
            _data = Some(Url::new(&url).generate());
        }
        QrData::Vcard {
            name,
            surname,
            tnumber,
            mnumber,
            email,
            url,
            company,
            jobtitle,
            fax,
            address,
            city,
            postcode,
            country,
        } => {
            _data = Some(
                Vcard::new(
                    &name, &surname, &tnumber, &mnumber, &email, &url, &company, &jobtitle, &fax,
                    &address, &city, &postcode, &country,
                )
                .generate(),
            );
        }

        QrData::Paypal { user } => {
            _data = Some(Paypal::new(&user).generate());
        }
        QrData::Phonenumber { number } => {
            _data = Some(Phonenumber::new(&number).generate());
        }
        QrData::Email {
            email,
            subject,
            message,
        } => {
            _data = Some(Email::new(&email, &subject, &message).generate());
        }
        QrData::Sms { number, message } => {
            _data = Some(Sms::new(&number, &message).generate());
        }
        QrData::Whatsapp { number, message } => {
            _data = Some(Whatsapp::new(&number, &message).generate());
        }
        QrData::Event {
            name,
            place,
            start,
            finish,
        } => {
            _data = Some(Event::new(&name, &place, start, finish).generate());
        }
    }
    _data
}
