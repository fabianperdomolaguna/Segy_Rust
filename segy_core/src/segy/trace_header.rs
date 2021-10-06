//use std::mem;
use std::io::prelude::*;
use std::io::BufWriter;
use crate::segy::construct::*;
use crate::bytes::{read,write};
use crate::utilities;

//let data = segy::trace_header::traceheader_iter::<u32>(&segy_struct, 72, bytes::read::read_ui32);
/*fn traceheader_iter<T> (segy_struct: &SegyFile, byte_pos: u64, f: fn(&mut std::fs::File) -> T) -> Vec<T>{
    let bytes_read: i64 = mem::size_of::<T>() as i64;
    let scalar: i64 = 240 - bytes_read + (segy_struct.number_samples * segy_struct.data_bytes) as i64;
    let mut data: Vec<T> = Vec::new();
    let fstream = &mut utilities::open_file(&segy_struct.filename, "read");
    fstream.seek(std::io::SeekFrom::Start(3600 + byte_pos)).expect("Could not read the file");
    for _i in 0..segy_struct.number_traces{
        data.push(f(fstream));
        fstream.seek(std::io::SeekFrom::Current(scalar)).expect("Could not read the file");
    }
    data
}*/

fn get_coord(segy_struct: &SegyFile) -> Vec::<(u32, u32)> {
    let mut coordinates = Vec::<(u32, u32)>::new();
    let scalar: i64 = 240 - 8 + (segy_struct.number_samples * segy_struct.data_bytes) as i64;
    let fstream = &mut utilities::open_file(&segy_struct.filename, "read");
    fstream.seek(std::io::SeekFrom::Start(3600 + 72)).expect("Could not read the file");
    for _i in 0..segy_struct.number_traces{
        coordinates.push((read::read_ui32(fstream), read::read_ui32(fstream)));
        fstream.seek(std::io::SeekFrom::Current(scalar)).expect("Could not read the file");
    }
    coordinates
}

pub fn print_coord(segy_struct: &SegyFile){
    let coordinates = get_coord(segy_struct);
    println!("{:-^30}", "");
    println!("{:^15} {:^15}", "X Coordinate", "Y Coordinate");
    for pair in coordinates{
        println!("{:^15} {:^15}", pair.0, pair.1);
    }
}

pub fn save_coord(segy_struct: &SegyFile){
    let coordinates = get_coord(segy_struct);
    let coord_filename = utilities::get_user_input("Enter header filename: ");
    let mut writer = BufWriter::new(utilities::open_file(&coord_filename, "create"));
    writeln!(writer,"X,Y").expect("Could not write in the file");
    for pair in coordinates{
        writeln!(writer,"{},{}", pair.0, pair.1).expect("Could not write in the file");
    }
}

fn write_coord(filename: &String, coordinates: &Vec::<(u32, u32)>, scalar: i64, byte_pos: u64){
    let segy_file = &mut utilities::open_file(&filename, "write");
    segy_file.seek(std::io::SeekFrom::Start(3600 + byte_pos)).expect("Could not read the file");
    for pair in coordinates{
        write::write_ui32(segy_file, pair.0);
        write::write_ui32(segy_file, pair.1);
        segy_file.seek(std::io::SeekFrom::Current(scalar)).expect("Could not read the file");
    }
}

pub fn replace_coord(segy_struct: &SegyFile){
    let coordinates = utilities::read_twocol_csv::<u32>("Enter coordinates filename: ");
    let scalar = 240 - 8 + (segy_struct.number_samples * segy_struct.data_bytes) as i64;
    write_coord(&segy_struct.filename, &coordinates, scalar, 72);
    write_coord(&segy_struct.filename, &coordinates, scalar, 80);
}
    