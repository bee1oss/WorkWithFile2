use std::fs::OpenOptions;
use std::io::{Read, stdin, Write};

fn main() {
    renamefile();
}

fn renamefile(){
    let path = "data.txt";

    std::fs::rename(path,"text.txt").expect("Error!");

}

fn openoptions(){
    let path = "data.txt";

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Error opening/creating file!");
    let mut file_data = String::new();
    f.read_to_string(&mut file_data).expect("Error reading file");
    println!("File Data:\n{}",file_data);

    println!("Enter something:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error getting user input");
    f.write_all(input.as_bytes()).expect("Error writing to file!");
}