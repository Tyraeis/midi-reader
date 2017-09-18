use std::io;

use errors::*;
use utils::*;
use track::*;

pub struct MidiFile {
    pub format: u16,
    pub division: u16,
    pub tracks: Vec<Track>
}

impl MidiFile {
    pub fn new<R>(input: &mut R) -> Result<MidiFile> where R: Iterator<Item=io::Result<u8>> {
        let mut midifile = MidiFile { format: 0, division: 0, tracks: Vec::new() };

        if read_header(input)? != "MThd" {
            return Err(ErrorKind::BrokenFileFormat.into());
        }

        let header_length = next_u32(input)?; // header length; should always be 6
        let format = next_u16(input)?;
        let num_tracks = next_u16(input)?;
        let division = next_u16(input)?;
        for _ in 0..(header_length - 6) as usize { next_byte(input)?; } // skip any extra bytes in the header

        let tracks = Vec::with_capacity(num_tracks as usize);
        for _ in 0..num_tracks {
            if read_header(input)? == "MTrk" {
                let length = next_u32(input)?;
            } else {
                let length = next_u32(input)?;
                for _ in 0..length { next_byte(input)?; }
            }
        }

        Ok(MidiFile {
            format, division, tracks
        })
    }
}