use super::Content;

pub struct Vcard {
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
}

impl Vcard {
    pub fn new(
        name: &str,
        surname: &str,
        tnumber: &str,
        mnumber: &str,
        email: &str,
        url: &str,
        company: &str,
        jobtitle: &str,
        fax: &str,
        address: &str,
        city: &str,
        postcode: &str,
        country: &str,
    ) -> Self {
        Self {
            name: name.into(),
            surname: surname.into(),
            tnumber: tnumber.into(),
            mnumber: mnumber.into(),
            email: email.into(),
            url: url.into(),
            company: company.into(),
            jobtitle: jobtitle.into(),
            fax: fax.into(),
            address: address.into(),
            city: city.into(),
            postcode: postcode.into(),
            country: country.into(),
        }
    }
}

impl Content for Vcard {
    fn generate(&self) -> String {
        format!(
            "BEGIN:VCARD
VERSION:3.0
N:{};{}
FN:{} {}
URL:{}
EMAIL:{}
TEL;TYPE=cell:{}
TEL;TYPE=home:{}
TEL;TYPE=fax: {}
ORG: {};;
TITLE:{}
ADR;TYPE=intl,work,postal,parcel:;;{};{};;{};{}


END:VCARD",
            self.surname,
            self.name,
            self.name,
            self.surname,
            self.url,
            self.email,
            self.mnumber,
            self.tnumber,
            self.fax,
            self.company,
            self.jobtitle,
            self.address,
            self.city,
            self.postcode,
            self.country
        )
        .clone()
    }
}
