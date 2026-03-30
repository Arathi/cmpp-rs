use std::fmt::Display;

pub const CMPP20: u8 = 0x20;
pub const CMPP30: u8 = 0x30;

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        let major = (value & 0xF0) >> 4;
        let minor = value & 0x0F;
        Self { major, minor }
    }
}

impl Into<u8> for Version {
    fn into(self) -> u8 {
        let mut value = self.major & 0x0F;
        value <<= 4;
        value |= self.minor & 0x0F;
        value
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CMPP {}.{}", self.major, self.minor)
    }
}

impl PartialEq<u8> for Version {
    fn eq(&self, other: &u8) -> bool {
        let value: u8 = self.clone().into();
        value == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cmpp20 = Version::from(CMPP20);
        assert_eq!(cmpp20.major, 2);
        assert_eq!(cmpp20.minor, 0);
        println!("{}", cmpp20);
    }
}
