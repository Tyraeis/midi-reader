use std::io;

use errors::*;
use utils::*;

#[derive(Debug, PartialEq)]
pub enum Event {
    Basic {
        typ: u8,
        channel: u8,
        data1: u8,
        data2: u8,
    },
    Sysex {
        typ: u8,
        data: Vec<u8>,
    },
    Meta {
        typ: u8,
        data: Vec<u8>,
    }
}

impl Event {
    pub fn parse<R>(input: &mut R) -> Result<Event> where R: Iterator<Item=io::Result<u8>> {
        let b1 = next_byte(input)?;
        let evt = match b1 {
            0xF0 | 0xF7 => {
                let length = read_varlen(input)?;
                let mut data = Vec::with_capacity(length as usize);

                for _ in 0..length {
                    data.push(next_byte(input)?);
                }

                Event::Sysex {
                    typ: b1,
                    data,
                }
            },
            0xFF => {
                let typ = next_byte(input)?;
                let length = read_varlen(input)?;
                let mut data = Vec::with_capacity(length as usize);

                for _ in 0..length {
                    data.push(next_byte(input)?);
                }

                Event::Meta {
                    typ,
                    data,
                }
            },
            _ => {
                Event::Basic {
                    typ: b1 & 0xF0,
                    channel: b1 & 0x0F,
                    data1: next_byte(input)?,
                    data2: next_byte(input)?,
                }
            }
        };
        Ok(evt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut data = (vec![0x91, 0x05, 0x7F]).into_iter().map(|x| Ok(x));
        let evt = Event::parse(&mut data).unwrap();

        if let Event::Basic {typ, channel, data1, data2} = evt {
            assert_eq!(0x90, typ);
            assert_eq!(0x01, channel);
            assert_eq!(0x05, data1);
            assert_eq!(0x7F, data2);
        } else {
            assert!(false, "Incorrect event type");
        }

        assert!(Event::parse(&mut data).is_err(), "Ensure parsing consumed the entire iterator");
    }

    #[test]
    fn test_sysex() {
        let mut data = (vec![0xF0, 0x03, 0x12, 0x34, 0x56, 0xF7, 0x00]).into_iter().map(|x| Ok(x));
        let evt1 = Event::parse(&mut data).unwrap();
        let evt2 = Event::parse(&mut data).unwrap();

        if let Event::Sysex {typ, data: d} = evt1 {
            assert_eq!(0xF0, typ, "Sysex event type 1");
            assert_eq!(vec![0x12, 0x34, 0x56], d);
        } else {
            assert!(false, "Incorrect event type");
        }

        if let Event::Sysex {typ, data: d} = evt2 {
            assert_eq!(0xF7, typ, "Sysex event type 2");
            assert_eq!(Vec::new() as Vec<u8>, d);
        } else {
            assert!(false, "Incorrect event type");
        }

        assert!(Event::parse(&mut data).is_err(), "Ensure parsing consumed the entire iterator");
    }

    #[test]
    fn test_meta() {
        let mut data = (vec![0xFF, 0x01, 0x03, 0xAB, 0xCD, 0xEF]).into_iter().map(|x| Ok(x));
        let evt = Event::parse(&mut data).unwrap();

        if let Event::Meta {typ, data} = evt {
            assert_eq!(0x01, typ);
            assert_eq!(vec![0xAB, 0xCD, 0xEF], data);
        } else {
            assert!(false, "Incorrect event type");
        }
    }
}