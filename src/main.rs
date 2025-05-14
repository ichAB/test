use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let path1="path/to/file.txt";
    let mut file = File::open(path1)?; 
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;  
    println!("contents: {}", contents);
    Ok(())
}