use std::fs::File;
use std::io::{Error, Read, ErrorKind};

fn main() {
    let file = "file.txt";

    match read_file(file) {
        Ok(file) => println!(" bu text fayilidan oqilgan {:?}", file),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(file) {
                Ok(f) => println!("file yaratildi"),
                Err(e) => panic!("fayil yaratishda xatolik bor {:?}", e),
            },
            _ => {panic!("qayta urining")}
        }
    }

}

fn read_file(file: &str) -> Result<String, Error> {
    let mut data = String::new();
    File::open(file)?.read_to_string(&mut data)?;
    Ok(data)
}
