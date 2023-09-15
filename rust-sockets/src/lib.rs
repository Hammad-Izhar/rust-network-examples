use byteorder::{BigEndian, ReadBytesExt};

pub enum Message {
    Guess(u16),
    GuessTooLow,
    GuessCorrect,
    GuessTooHigh,
}

impl Message {
    pub fn marshal(self) -> Vec<u8> {
        match self {
            Self::Guess(x) => {
                let mut buffer: Vec<u8> = vec![0; 3];
                buffer[0] = 0;
                buffer[1..].copy_from_slice(&x.to_be_bytes());
                buffer
            }
            Self::GuessTooLow => vec![1; 1],
            Self::GuessCorrect => vec![2, 1],
            Self::GuessTooHigh => vec![3, 1],
        }
    }

    pub fn unmarshal(bytes: &Vec<u8>) -> Option<Self> {
        match bytes.split_first() {
            Some((0, mut last_bytes)) => {
                Some(Self::Guess(last_bytes.read_u16::<BigEndian>().unwrap()))
            }
            Some((1, _)) => Some(Self::GuessTooLow),
            Some((2, _)) => Some(Self::GuessCorrect),
            Some((3, _)) => Some(Self::GuessTooHigh),
            _ => None,
        }
    }
}
