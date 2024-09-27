use std::fs::File;
use std::io::{self, Read, ErrorKind, Error};


fn main() {
    //operatori panic! makrosi dasturda kritic xatolik yuz berganida ishlatiladi. Bu funksiya chaqirilganda, dastur darhol to'xtaydi.
    panic!("Xatolik yuz berdi");
    let list = vec![1, 2, 3, 4, 5];
    list[3];


    // Result
    let file = "file.txt";

    match read_file(file) {
        Ok(data) => println!("Bu fayil ichidagi yozuv {:?}", data),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(file) {
                Ok(file) => file,
                Err(err) => panic!("Fayil ochishda xatolik bor {:?}", err),
                
            },
            _ => panic!("Fayilni ochib bolmadi")
        }
    };

}

fn read_file(file:&str) -> Result<String, Error> {
    let f = File::open(file);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), //agar file bolsa filni qaytaradi bolmasa error qaytradi
    };
    // file oqish
    let mut data = String::new();
    match f.read_to_string(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(e)
    }
}