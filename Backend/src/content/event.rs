use super::Content;

pub struct Event {
    name: String,
    place: String,
    start: isize,
    finish: isize,
}

impl Event {
    pub fn new(name: &str, place: &str, start: isize, finish: isize) -> Self {
        Self {
            name: name.into(),
            place: place.into(),
            start,
            finish,
        }
    }
}

impl Content for Event {
    fn generate(&self) -> String {
        format!(
            "BEGIN:VEVENT
SUMMARY:{}
LOCATION:{}
DTSTART:{}
DTEND:{}
SUMMARY:{}
END:VEVENT",
            self.name, self.place, self.start, self.finish, self.name
        )
    }
}

// BEGIN:VEVENT
// SUMMARY:name
// LOCATION:ort
// DTSTART:20231006T160900
// DTEND:20231006T160900
// END:VEVENT
