use std::io;
use std::io::Read;
use std::fs::File;

pub fn read_username_from_file1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename);
    let mut f = match f{
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => return Err(error),
    }
}

pub fn read_username_from_file2(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_username_from_file3(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}