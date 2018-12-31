use chrono::{DateTime, NaiveDate, Utc};
use nom::types::CompleteStr;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum EventType {
    StartsShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
pub struct Event {
    pub time: DateTime<Utc>,
    pub kind: EventType,
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.time == other.time
    }
}

impl Eq for Event {}

fn from_dec(input: CompleteStr) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(&input, 10)
}

fn is_decimal_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(integer<CompleteStr, u32>,
    map_res!(take_while!(is_decimal_digit), from_dec)
);

named!(datetime<CompleteStr, DateTime<Utc>>,
    do_parse!(
            tag!("[") >>
        year: integer >>
            tag!("-") >>
        month: integer >>
            tag!("-") >>
        day:  integer >>
            tag!(" ") >>
        hour: integer >>
            tag!(":") >>
        minute: integer >>
            tag!("]") >>
        (DateTime::<Utc>::from_utc(NaiveDate::from_ymd(year as i32, month, day).and_hms(hour, minute, 0), Utc))
    )
);

named!(parse_begin_shift<CompleteStr, EventType>,
    do_parse!(
            tag!("Guard #") >>
         id: integer >>
             tag!(" begins shift") >>
        (EventType::StartsShift(id))
    )
);

impl EventType {
    fn from_str(input: CompleteStr) -> Result<EventType, std::io::Error> {
        match input.as_ref() {
            "falls asleep" => Ok(EventType::FallsAsleep),
            "wakes up" => Ok(EventType::WakesUp),
            _str => {
                let (_, event) = parse_begin_shift(input).unwrap();
                Ok(event)
            }
        }
    }
}

fn is_valid_string(c: char) -> bool {
    c.is_alphanumeric() || c.is_whitespace() || c == '#'
}

named!(event_type<CompleteStr, EventType>,
    map_res!(take_while!(is_valid_string), EventType::from_str)
);

named!(pub guard_event<CompleteStr, Event>,
    do_parse!(
        time: datetime >>
            tag!(" ") >>
        kind: event_type >>
        (Event {time: time, kind: kind})
    )
);
