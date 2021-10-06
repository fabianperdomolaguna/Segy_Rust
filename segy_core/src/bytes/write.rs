use std::io::prelude::*;

pub fn write_ui32(fstream: &mut std::fs::File, value: u32){
    let bytes = [((value >> 24) & 0xff) as u8,
    	((value >> 16) & 0xff) as u8,
    	((value >>  8) & 0xff) as u8,
    	((value >>  0) & 0xff) as u8];
    fstream.write(&bytes).unwrap();
}