use std::io::prelude::*;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::str::FromStr;

pub fn get_user_input(message: &str) -> String{
    let mut input = String::new();
    print!("{}", message);
    let _ = io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Not a valid string");
    input
}

pub fn get_user_option (message: &str) -> u16 {
    let mut input = String::new();
    print!("{}", message);
    let _ = io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let input_number: u16 = input.trim().parse().expect("Not a valid number");
    input_number
}

pub fn open_file(filename: &String, option: &str) -> std::fs::File{
    let fstream = match option{
        "read" => OpenOptions::new().read(true).open(filename.trim()).expect("Can't open file!"),
        "write" => OpenOptions::new().read(true).write(true).create(true).open(filename.trim()).expect("Can't open file!"),
        _ => std::fs::File::create(filename.trim()).expect("Unable to create the file")
    };
    fstream
}

pub fn read_twocol_csv<T: FromStr>(message: &str) -> Vec::<(T, T)> {
    let mut pairs = Vec::<(T, T)>::new();
    let filename = get_user_input(message);
    let reader = BufReader::new(open_file(&filename, "read"));
    for line in reader.lines().skip(1){
        let line_string =  line.unwrap();
        let mut values = line_string.split(',').map(|x| T::from_str(x).ok().unwrap()).collect::<Vec<_>>();
        pairs.push((values.remove(0), values.remove(0)));
    }
    pairs
}