use std::io;

use errors::*;

pub fn next_byte<R>(bytes: &mut R) -> Result<u8> where R: Iterator<Item=io::Result<u8>> {
    if let Some(b) = bytes.next() {
        Ok(b?)
    } else {
        Err(ErrorKind::BrokenFileFormat.into())
    }
}

pub fn next_u16<R>(bytes: &mut R) -> Result<u16> where R: Iterator<Item=io::Result<u8>> {
    Ok(((next_byte(bytes)? as u16) << 8) + (next_byte(bytes)? as u16))
}

pub fn next_u32<R>(bytes: &mut R) -> Result<u32> where R: Iterator<Item=io::Result<u8>> {
    Ok(((next_byte(bytes)? as u32) << 24) + ((next_byte(bytes)? as u32) << 16) + ((next_byte(bytes)? as u32) << 8) + (next_byte(bytes)? as u32))
}

pub fn check_header<R>(input: &mut R, header: &str) -> Result<bool> where R: Iterator<Item=io::Result<u8>> {
    let bytes = [next_byte(input)?, next_byte(input)?, next_byte(input)?, next_byte(input)?];
    Ok(bytes == header.as_bytes())
}

pub fn read_header<R>(input: &mut R) -> Result<String> where R: Iterator<Item=io::Result<u8>> {
    let mut st = String::with_capacity(4);
    for _ in 0..4 {
        st.push(next_byte(input)?.into());
    }
    Ok(st)
}

pub fn try_read_varlen<R>(input: &mut R) -> Result<Option<u32>> where R: Iterator<Item=io::Result<u8>> {
    let mut data: u32 = 0;
    let first_byte = input.next();
    if first_byte.is_none() {
        return Ok(None);
    }

    let mut cur_byte = first_byte.unwrap()?;
    // check the most significant bit to see if there are more bytes to read
    while (cur_byte >> 7) & 1 == 1 {
        // append the seven least significant bits
        data = (data << 7) + ((cur_byte & 0b01111111) as u32);
        cur_byte = next_byte(input)?;
    }
    data = (data << 7) + (cur_byte as u32);
    Ok(Some(data))
}

pub fn read_varlen<R>(input: &mut R) -> Result<u32> where R: Iterator<Item=io::Result<u8>> {
    let data = try_read_varlen(input)?;
    data.ok_or(ErrorKind::BrokenFileFormat.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_header() {
        let mut data1 = (vec![0x4d, 0x54, 0x68, 0x64]).into_iter().map(|x| Ok(x)); // MThd
        assert!(check_header(&mut data1, "MThd").unwrap());

        let mut data2 = (vec![0x4d, 0x00, 0x68, 0x64]).into_iter().map(|x| Ok(x)); // M<nul>hd
        assert!(!check_header(&mut data2, "MThd").unwrap());
    }

    #[test]
    fn test_read_header() {
        let mut data = (vec![0x4D, 0x54, 0x68, 0x64, 0x4D, 0x54, 0x72, 0x6B]).into_iter().map(|x| Ok(x)); // MThdMTrk
        assert_eq!("MThd", read_header(&mut data).unwrap());
        assert_eq!("MTrk", read_header(&mut data).unwrap());
    }

    #[test]
    fn test_read_varlen() {
        let mut data = (vec![0b10000001, 0b10000001, 0b00000001]).into_iter().map(|x| Ok(x));
        let val = read_varlen(&mut data).unwrap();

        assert_eq!(0b100000010000001, val);
    }
}