use std::string::String;
use std::fs::File;
use std::io::*;
pub fn create_file_with_data(path: String, data: String) -> Result<usize> {
    let mut file = File::create(path).expect("Error while creating file");
    let buffer = data.into_bytes();
    file.write(&buffer).expect("No data written");
    drop(file);
    Ok(buffer.len())
}

pub fn read_file(path: &String) -> String {
    use std::mem::*;
    let mut file = File::open(&path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read error");
    drop(file);
    contents
}
