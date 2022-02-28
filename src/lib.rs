use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use encoding_rs::*;


pub fn write(path: &str, contents: &str, codec: &str) -> Result<(), Box<dyn Error>> {
    let encoding = Encoding::for_label(codec.as_bytes()).unwrap();
    
    let mut file = File::create(path)?;
    let (cow, _encoding_used, _had_errors) = encoding.encode(contents);
    let byte_code = &cow[..]; 
    file.write_all(byte_code)?;

    Ok(())
}

pub fn read(path: &str, codec: &str) -> Result<String, Box<dyn Error>> {
    let encoding = Encoding::for_label(codec.as_bytes()).unwrap();

    let mut file = File::open(path)?;
    let mut buffer = Vec::new(); 
    file.read_to_end(&mut buffer)?;

    let (cow, _encoding_used, _had_errors) = encoding.decode(&buffer);
    let byte_code = String::from(&cow[..]);

    Ok(byte_code)
}


#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read_windows_1252() {
        let test_file = "test.txt";
        let source = "ÁáAaBbCc";
        let codec = "latin1";

        write(test_file, source, codec);
        let result = read(test_file, codec).unwrap();
        std::fs::remove_file(test_file);

        assert_eq!(result, source);
    }
}
