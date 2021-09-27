use std::io::prelude::*;
use std::io::BufReader;
use crate::segy::construct::*;
use crate::utilities;

static E2A: [u8; 256] = [
    0,  1,  2,  3,156,  9,134,127,151,141,142, 11, 12, 13, 14, 15,
    16, 17, 18, 19,157,133,  8,135, 24, 25,146,143, 28, 29, 30, 31,
	128,129,130,131,132, 10, 23, 27,136,137,138,139,140,  5,  6,  7,
	144,145, 22,147,148,149,150,  4,152,153,154,155, 20, 21,158, 26,
	32,160,161,162,163,164,165,166,167,168, 91, 46, 60, 40, 43, 33,
	38,169,170,171,172,173,174,175,176,177, 93, 36, 42, 41, 59, 94,
	45, 47,178,179,180,181,182,183,184,185,124, 44, 37, 95, 62, 63,
	186,187,188,189,190,191,192,193,194, 96, 58, 35, 64, 39, 61, 34,
	195, 97, 98, 99,100,101,102,103,104,105,196,197,198,199,200,201,
	202,106,107,108,109,110,111,112,113,114,203,204,205,206,207,208,
	209,126,115,116,117,118,119,120,121,122,210,211,212,213,214,215,
	216,217,218,219,220,221,222,223,224,225,226,227,228,229,230,231,
	123, 65, 66, 67, 68, 69, 70, 71, 72, 73,232,233,234,235,236,237,
	125, 74, 75, 76, 77, 78, 79, 80, 81, 82,238,239,240,241,242,243,
	92,159, 83, 84, 85, 86, 87, 88, 89, 90,244,245,246,247,248,249,
	48, 49, 50, 51, 52, 53, 54, 55, 56, 57,250,251,252,253,254,255
];

static A2E: [u8; 256] = [
	0, 1, 2, 3, 55, 45, 46, 47, 22, 5, 37, 11, 12, 13, 14, 15,
	16, 17, 18, 19, 60, 61, 50, 38, 24, 25, 63, 39, 28, 29,30,31,
	64, 79,127,123, 91,108,80,125, 77, 93, 92, 78, 107, 96,75,97,
	240,241,242,243,244,245,246,247,248,249,122,94,76,126,110,111,
	124,193,194,195,196,197,198,199,200,201,209,210,211,212,213,214,
	215,216,217,226,227,228,229,230,231,232,233,74,224,90,95,109,
	121,129,130,131,132,133,134,135,136,137,145,146,147,148,149,150,
	151,152,153,162,163,164,165,166,167,168,169,192,106,208,161,7,
	32, 33, 34, 35, 36, 21, 6, 23, 40, 41, 42, 43, 44, 9, 10, 27,
	48, 49, 26, 51, 52, 53, 54, 8, 56, 57, 58, 59, 4, 20, 62, 225,
	65, 66, 67, 68, 69, 70, 71, 72, 73, 81, 82, 83, 84, 85, 86, 87,
	88, 89, 98, 99,100,101,102,103,104,105,112,113,114,115,116,117,
	118,119,120,128,138,139,140,141,142,143,144,154,155,156,157,158,
	159,160,170,171,172,173,174,175,176,177,178,179,180,181,182,183,
	184,185,186,187,188,189,190,191,202,203,204,205,206,207,218,219,
	220,221,222,223,234,235,236,237,238,239,250,251,252,253,254,255
];

fn get_textual_header(filename: &String) -> Vec<u8>{
	let fstream = &mut utilities::open_file(filename, "read");
	let mut ebcdic_header = vec![0u8; 3200];
	fstream.read(&mut ebcdic_header).expect("Unable to read the textual header");
	let ascii_header = ebcdic_header.iter().map(|&x| E2A[x as usize]).collect::<Vec<_>>();
	ascii_header
}

pub fn print_textual_header(segy_struct: &SegyFile){
	let ascii_header = get_textual_header(&segy_struct.filename);
	let textual_header = String::from_utf8_lossy(&ascii_header);
	println!("{:-^80}", "Textual Header");
    for i in (0..3200).step_by(80){
        println!("{}", &textual_header[i..i+80])
    }
}

pub fn save_textual_header(segy_struct: &SegyFile){
	let ascii_header = get_textual_header(&segy_struct.filename);
	let header_filename = utilities::get_user_input("Enter filename:");
    let ofstream = &mut utilities::open_file(&header_filename, "create");
    for i in (0..3200).step_by(80){
        ofstream.write(&ascii_header[i..i+80]).unwrap();
        ofstream.write(b"\n").unwrap();
    }
}

pub fn replace_textual_header(segy_struct: &SegyFile){
	let fstream = &mut utilities::open_file(&segy_struct.filename, "write");
	let header_filename = utilities::get_user_input("Enter header filename: ");
	let reader = BufReader::new(utilities::open_file(&header_filename, "read"));
	for line in reader.lines(){
		let line_string = line.unwrap();
		let ebcdic_bytes = line_string.as_bytes().iter().map(|&x| A2E[x as usize]).collect::<Vec<_>>();
		fstream.write(&ebcdic_bytes).unwrap();
	}
}