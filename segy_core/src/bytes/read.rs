use std::io::prelude::*;

fn swap4_bytes(bytes: &u32) -> u32{
    (bytes >> 24) | (bytes << 24) | ((bytes >> 8) & 0xff00) | ((bytes & 0xff00) << 8)
}

pub fn read_i32(fstream: &mut std::fs::File) -> i32{
    let mut buf = vec![0u8; 4];
    fstream.read(&mut buf).expect("Oops!");
    ((buf[0] as i32) << 24) + ((buf[1] as i32) << 16) + 
        ((buf[2] as i32) << 8) + (buf[3] as i32)
}

pub fn read_ui32(fstream: &mut std::fs::File) -> u32{
    let mut buf = vec![0u8; 4];
    fstream.read(&mut buf).expect("Oops!");
    ((buf[0] as u32) << 24) + ((buf[1] as u32) << 16) + 
        ((buf[2] as u32) << 8) + (buf[3] as u32)
}

pub fn read_i16(fstream: &mut std::fs::File) -> i16{
    let mut buf = vec![0u8; 2];
    fstream.read(&mut buf).expect("Oops!");
    ((buf[0] as i16) << 8) + (buf[1] as i16)
}

pub fn read_ui16(fstream: &mut std::fs::File) -> u16{
    let mut buf = vec![0u8; 2];
    fstream.read(&mut buf).expect("Oops!");
    ((buf[0] as u16) << 8) + (buf[1] as u16)
}

pub fn read_ui8(fstream: &mut std::fs::File) -> u8{
    let mut buf = vec![0u8; 1];
    fstream.read(&mut buf).expect("Oops!");
    buf[0]
}

pub fn read_ibm(fstream: &mut std::fs::File) -> f32{
    let mut buf: [u8; 4] = [0; 4];
    fstream.read(&mut buf).expect("Oops!");
    let bytes = u32::from_ne_bytes(buf);
    let mut bytes_swap = swap4_bytes(&bytes);
    let sign = (bytes_swap >> 31) as i16; bytes_swap <<= 1;
    let exponent = (bytes_swap >> 25) as i16; bytes_swap <<= 7;
    let mantissa = ((bytes_swap >> 8) as f32) / (16777216 as f32);
    ((1 - 2 * sign) as f32) * mantissa * f32::powf(16 as f32, (exponent-64) as f32)
}