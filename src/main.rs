use segy_core::*;

fn textual_header_menu(segy_struct: &segy::construct::SegyFile){
    println!("{:-^30}", "Menu");
    println!("{}", "1. Print textual header");
    println!("{}", "2. Save textual header");
    println!("{}", "3. Replace textual header");
    println!("{:-^30}", "");

    match utilities::get_user_option("Enter option number: "){
        1 => segy::textual_header::print_textual_header(segy_struct),
        2 => segy::textual_header::save_textual_header(segy_struct),
        3 => segy::textual_header::replace_textual_header(segy_struct),
        _ => println!("{}", "Option not found")
    }

}

fn trace_header_menu(segy_struct: &segy::construct::SegyFile){
    println!("{:-^30}", "Menu");
    println!("{}", "1. Print XY coordinates");
    println!("{}", "2. Save XY coordinates");
    println!("{}", "3. Replace coordinates");
    println!("{:-^30}", "");
}

fn main_menu(segy_struct: &segy::construct::SegyFile){
    println!("{:-^30}", "Main Menu");
    println!("{}", "1. SEG-Y file summary");
    println!("{}", "2. Textual header");
    println!("{}", "3. Trace header");
    println!("{:-^30}", "");

    match utilities::get_user_option("Enter option number: "){
        1 => segy::construct::print_main_params(segy_struct),
        2 => textual_header_menu(segy_struct),
        3 => trace_header_menu(segy_struct),
        _ => println!("{}", "Option not found")
    }
}

fn main() {
    let segy_struct = segy::construct::segy_struct_construct();
    main_menu(&segy_struct);
}
