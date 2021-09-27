use std::mem;
use std::io::prelude::*;
use crate::segy::construct::*;
use crate::utilities;

//let data = segy::trace_header::traceheader_iter::<u32>(&segy_struct, 72, bytes::read::read_ui32);
pub fn traceheader_iter<T> (segy_struct: &SegyFile, byte_pos: u64, f: fn(&mut std::fs::File) -> T) -> Vec<T>{
    let bytes_read: i64 = mem::size_of::<T>() as i64;
    let scalar: i64 = 240 - bytes_read + (segy_struct.number_samples * segy_struct.data_bytes) as i64;
    let mut data: Vec<T> = Vec::new();
    let fstream = &mut utilities::open_file(&segy_struct.filename, "read");
    fstream.seek(std::io::SeekFrom::Start(3600 + byte_pos)).expect("Could not read the file");
    for _i in 0..segy_struct.number_traces{
        data.push(f(fstream));
        fstream.seek(std::io::SeekFrom::Current(scalar as i64)).expect("Could not read the file");
    }
    data
}