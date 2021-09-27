use std::io::prelude::*;
use std::collections::HashMap;
use crate::bytes::read as bytes;
use crate::utilities;

#[derive(Default)]
pub struct SegyFile{
    pub filename: String,
    filesize: u64,
    pub number_samples: u16,
    data_format: i16,
    pub data_bytes: u16,
    pub number_traces: u64
}

fn get_filesize(fstream: &mut std::fs::File) -> u64{
    let filesize = fstream.metadata().unwrap().len();
    filesize
}

fn get_number_samples(fstream: &mut std::fs::File) -> u16{
    fstream.seek(std::io::SeekFrom::Start(3220)).expect("Could not read the file");
    bytes::read_ui16(fstream)
}

fn get_data_format(fstream: &mut std::fs::File) -> i16{
    fstream.seek(std::io::SeekFrom::Start(3224)).expect("Could not read the file");
    bytes::read_i16(fstream)
}

fn get_data_bytes(data_format: i16) -> u16{
    let mut format: HashMap<i16,u16> = [(1,4), (2,4), (3,2), (4,4), (5,4), (6,8), (7,3),
        (8,1), (9,8), (10,4), (11,2), (12,8), (15,3), (16,1)].iter().cloned().collect();
    format.remove(&data_format).unwrap()
}

fn get_number_traces(filesize: u64, number_samples: u16, data_bytes: u16) -> u64{
    (filesize - 3600)/ (240 + (number_samples as u64) * (data_bytes as u64))
}

pub fn segy_struct_construct() -> SegyFile{
    let mut segy_struct = SegyFile{
        filename: utilities::get_user_input("Enter filename: "),
        ..Default::default()
    };
    let fstream = &mut utilities::open_file(&segy_struct.filename, "read");
    segy_struct.filesize = get_filesize(fstream);
    segy_struct.number_samples = get_number_samples(fstream);
    segy_struct.data_format = get_data_format(fstream);
    segy_struct.data_bytes = get_data_bytes(segy_struct.data_format);
    segy_struct.number_traces = get_number_traces(segy_struct.filesize, 
        segy_struct.number_samples, segy_struct.data_bytes);
    segy_struct
}

fn get_main_params(fstream: &mut std::fs::File) -> Vec<i32>{
    let bytes_position: [(i16,&str); 8] = [(3204,"i32"), (3212,"i16"), (3216,"ui16"), (3228,"i16"),
        (3254,"i16"), (3256,"i16"), (3500,"ui8"), (3501,"ui8")];
    let mut main_params = Vec::<i32>::new();
    for (key, value) in bytes_position.iter(){
        fstream.seek(std::io::SeekFrom::Start(*key as u64)).expect("Could not read the file");
        match value.as_ref() {
            "i32" => main_params.push(bytes::read_i32(fstream)),
            "i16" => main_params.push(bytes::read_i16(fstream) as i32),
            "ui16" => main_params.push(bytes::read_ui16(fstream) as i32),
            "ui8" => main_params.push(bytes::read_ui8(fstream) as i32),
            _ => println!("{}", "Option not found")
        }
    }
    main_params
}

pub fn print_main_params(segy_struct: &SegyFile){
    let fstream = &mut utilities::open_file(&segy_struct.filename, "read");

    let data_format_name: HashMap<i16,&str> = [(1,"4-byte IBM floating point"), 
        (2,"4-byte, two's complement integer"), (3,"2-byte, two's complement integer"),
        (4,"4-byte fixed-point with gain"), (5,"4-byte IEEE floating-point"), 
        (6,"8-byte IEEE floating-point"), (7,"3-byte two’s complement integer"),
        (8,"1-byte, two's complement integer"), (9,"8-byte, two's complement integer"),
        (10,"4-byte, unsigned integer"), (11,"2-byte, unsigned integer"), (12,"8-byte, unsigned integer"),
        (15,"3-byte, unsigned integer"), (16,"1-byte, unsigned integer")].iter().cloned().collect();

    let sort_code: HashMap<i32,&str> = [(0,"Unknown"), (1,"As recorded"),
        (2,"CDP emsemble"), (3,"Single fold continuous profile"), (4,"Horizontally stacked"),
        (5,"Common source point"), (6,"Common receiver point"), (7,"Common offset point"),
        (8,"Common mid-point"), (9,"Common conversion point")].iter().cloned().collect();

    let measure_system: HashMap<i32,&str> = [(1,"Meters"), (2,"Feets")].iter().cloned().collect();

    let polarity: HashMap<i32,&str> = [(0,"Unknown"), (1,"European"), 
        (2,"American")].iter().cloned().collect();

    let params= get_main_params(fstream);
    println!("{:-^68}", "SEG-Y Info");
    println!("{:>34} {}", "Filename:", segy_struct.filename.replace("\r\n", ""));
    println!("{:>34} {:.2} MB", "Filesize:", (segy_struct.filesize as f32)/1000000.);
    println!("{:>34} {}", "Line number:", params[0]);
    println!("{:>34} {}", "Number of data traces per record:", params[1]);
    println!("{:>34} {} (us)", "Sample interval:", params[2]);
    println!("{:>34} {}", "Number of samples per data trace:", segy_struct.number_samples);
    println!("{:>34} {}", "Data sample format:", data_format_name.get(&segy_struct.data_format).unwrap());
    println!("{:>34} {} ({})", "Trace sorting code:", params[3], sort_code.get(&params[3]).unwrap());
    println!("{:>34} {} ({})", "Measurement system:", params[4], measure_system.get(&params[4]).unwrap());
    println!("{:>34} {} ({})", "Signal polarity:", params[5], polarity.get(&params[5]).unwrap());
    println!("{:>34} {}.{}", "SEG-Y Rev:", params[6], params[7]);
    println!("{:>34} {}", "Number of traces:", segy_struct.number_traces);
    println!("{:>34} {} (s)", "Time length:", (params[2] as f32) * 
        ((segy_struct.number_samples as f32) - 1.) / 1000000.);
}