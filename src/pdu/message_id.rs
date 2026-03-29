use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct MessageId {
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub gateway_id: u32,
    pub sequence: u16,
}

impl From<u64> for MessageId {
    fn from(value: u64) -> Self {
        let mut value = value;

        // 16-1 16bits
        let sequence = (value & 0x0000_ffff) as u16;
        value >>= 16;

        // 38-17 22bits
        let gateway_id = (value & 0x001f_ffff) as u32;
        value >>= 22;

        // 44-39 6bits
        let second = (value & 0b0011_1111) as u8;
        value >>= 6;

        // 50-45 6bits
        let minute = (value & 0b0011_1111) as u8;
        value >>= 6;

        // 55-51 5bits
        let hour = (value & 0b0001_1111) as u8;
        value >>= 5;

        // 60-56 5bits
        let day = (value & 0b0001_1111) as u8;
        value >>= 5;

        // 64-61 4bits
        let month = (value & 0b0000_1111) as u8;

        MessageId {
            month,
            day,
            hour,
            minute,
            second,
            gateway_id,
            sequence,
        }
    }
}

impl Into<u64> for MessageId {
    fn into(self) -> u64 {
        // 64-61 4bits
        let mut value: u64 = self.month as u64;

        // 60-56 5bits
        value <<= 5;
        value |= self.day as u64;

        // 55-51 5bits
        value <<= 5;
        value |= self.hour as u64;

        // 50-45 6bits
        value <<= 6;
        value |= self.minute as u64;

        // 44-39 6bits
        value <<= 6;
        value |= self.second as u64;

        // 38-17 22bits
        value <<= 22;
        value |= self.gateway_id as u64;

        // 16-1 16bits
        value <<= 16;
        value |= self.sequence as u64;

        value
    }
}

impl Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}{:02}{:02}{:02}{:02}-{:07}-{:05}",
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.gateway_id,
            self.sequence,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MSG_DISP: &str = "0328224230-0328965-20560";
    const MSG_ID: u64 = 4493087768012214352;

    #[test]
    fn test_display() {
        let id = MessageId {
            month: 3,
            day: 28,
            hour: 22,
            minute: 42,
            second: 30,
            gateway_id: 0x050505,
            sequence: 0x5050,
        };
        println!("{:?}", &id);
        println!("{}", &id);

        let display = format!("{}", &id);
        assert_eq!(display, MSG_DISP);
    }

    #[test]
    fn test_into_u64() {
        let id = MessageId {
            month: 3,
            day: 28,
            hour: 22,
            minute: 42,
            second: 30,
            gateway_id: 0x050505,
            sequence: 0x5050,
        };
        println!("{:?}", &id);

        let id_value: u64 = id.into();
        println!("{:064b}", id_value);
        println!("{:019}", id_value);
    }

    #[test]
    fn test_from_u64() {
        let id: MessageId = MessageId::from(MSG_ID);

        println!("month = {}", id.month);
        assert_eq!(id.month, 3);

        println!("day = {}", id.day);
        assert_eq!(id.day, 28);

        println!("hour = {}", id.hour);
        assert_eq!(id.hour, 22);

        println!("minute = {}", id.minute);
        assert_eq!(id.minute, 42);

        println!("second = {}", id.second);
        assert_eq!(id.second, 30);

        println!("gateway_id = {}", id.gateway_id);
        assert_eq!(id.gateway_id, 0x050505);

        println!("sequence = {}", id.sequence);
        assert_eq!(id.sequence, 0x5050);
    }
}
