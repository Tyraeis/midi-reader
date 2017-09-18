use std::io;

use event::*;
use errors::*;
use utils::*;

pub struct Track {
    pub events: Vec<(u32, Event)>,
}

impl Track {
    pub fn new<R>(input: &mut R) -> Result<Track> where R: Iterator<Item=io::Result<u8>> {
        let mut events: Vec<(u32, Event)> = Vec::new();
        while let Some(dt) = try_read_varlen(input)? {
            events.push((dt, Event::parse(input)?));
        }

        Ok(Track { events })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_parsing() {
        let mut data = (vec![
            0x00, 0x90, 0x40, 0x50, // dt 0x00: note on; key: 0x40, vel: 0x50
            0x10, 0x80, 0x40, 0x00, // dt 0x10: note off; key: 0x40, vel: 0x00
            0xFF, 0x7F, 0xF0, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F, // dt 0x7FFF: 0xF0 sysex; data: "hello"
        ]).into_iter().map(|x| Ok(x));

        let trk = Track::new(&mut data).unwrap();

        assert_eq!((0x00, Event::Basic {
            typ: 0x90,
            channel: 0x00,
            data1: 0x40,
            data2: 0x50
        }), trk.events[0]);

        assert_eq!((0x10, Event::Basic {
            typ: 0x80,
            channel: 0x00,
            data1: 0x40,
            data2: 0x00
        }), trk.events[1]);

        assert_eq!((0x3FFF, Event::Sysex {
            typ: 0xF0,
            data: vec![0x68, 0x65, 0x6C, 0x6C, 0x6F]
        }), trk.events[2]);
    }
}