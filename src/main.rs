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
// const THRB: usize = 32;

type Group = [u8; FOUR_KB];

fn get_bytes_from_asset(reader: &mut BufReader<File>) -> Result<Vec<Group>, Box<dyn Error>> {
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

fn open_file_create_reader(path_str: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let path = Path::new(path_str);

    if !path.exists() {
        return Err("File does not exist".into());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn main() {
    let example = "assets/example.jpg";
    let mut reader = open_file_create_reader(example).unwrap();

    let bytes = get_bytes_from_asset(&mut reader).unwrap();
    let res = bytes.iter().map(hex::encode).collect::<Vec<String>>();
    dbg!(res);
}
