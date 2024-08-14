use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

/*
4 bytes positional (CRC?)
[10... begining
[01... ending
[00.. middle
[11 roothash

*/

const FOUR_KB: usize = 4 * 1024;

pub type Group = [u8; FOUR_KB];

// #[no_mangle]
// pub extern "C" fn some_func() {
//     println!("Hello from Rust");
// }

pub extern "C" fn get_bytes_from_asset<T>(
    reader: &mut BufReader<T>,
) -> Result<Vec<Group>, Box<dyn Error>>
where
    T: Read,
{
    let mut output = vec![];

    // let mut is_start = true;
    // let mut is_end = false;

    loop {
        let mut buffer = [0; FOUR_KB];
        let bytes = reader.read(&mut buffer)?;

        if bytes == 0 {
            break;
        }

        output.push(buffer);
    }

    Ok(output)
}

pub fn open_file_create_reader(path_str: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let path = Path::new(path_str);

    if !path.exists() {
        return Err("File does not exist".into());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeFile {}

    impl Read for FakeFile {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            Ok(0)
        }
    }

    #[test]
    fn test_get_bytes_from_asset() {
        let mock_file: Group = [0; FOUR_KB];
    }
}
