use std::fmt::Display;

use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Timestamp {
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Timestamp {
    pub fn now() -> Self {
        Timestamp::from(Local::now())
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}{:02}{:02}{:02}{:02}",
            self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

impl From<u32> for Timestamp {
    fn from(value: u32) -> Self {
        let mut value = value;
        let second = (value % 100) as u8;
        value /= 100;
        let minute = (value % 100) as u8;
        value /= 100;
        let hour = (value % 100) as u8;
        value /= 100;
        let day = (value % 100) as u8;
        value /= 100;
        let month = (value % 100) as u8;

        Self {
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl From<DateTime<Local>> for Timestamp {
    fn from(value: DateTime<Local>) -> Self {
        let datetime: String = value.format("%m%d%H%M%S").to_string();
        let datetime_u32: u32 = datetime.parse().unwrap();
        Timestamp::from(datetime_u32)
    }
}

impl Into<u32> for Timestamp {
    fn into(self) -> u32 {
        let mut value = self.month as u32;
        value *= 100;
        value += self.day as u32;
        value *= 100;
        value += self.hour as u32;
        value *= 100;
        value += self.minute as u32;
        value *= 100;
        value += self.second as u32;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let timestamp = Timestamp::now();
        println!("{:?}", &timestamp);
        println!("{}", &timestamp);
        let timestamp_u32: u32 = timestamp.into();
        println!("{:010}", &timestamp_u32);
    }
}
