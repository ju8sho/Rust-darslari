use std::fs::{File, OpenOptions};
use std::io::{Read, stdin, Write};

fn main() {
    // oddiy usullari
    let file = "Test.txt";
    // let mut f = File::create(file).expect("Fayil yaratishda xatolik bor");
    // f.write_all("bu rast tili fayillar bilan ishlash".as_bytes()).expect("Fayil yozishda xato yuz berdi");

    let mut file_open = File::open(file).expect("xato");
    let mut file_data = String::new();
    file_open.read_to_string(&mut file_data).expect("fayilni oqishda xatolik");
    println!("{}", file_data);


    let mut fayl = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file)
        .expect("Fayil ochishda xatolik bor");

    let mut file_data = String::new();
    fayl.read_to_string(&mut file_data).expect("fayilni oqishda xatolik bor");

    println!("Fayil ichidagi qiymatlar: \n{}", file_data);
    println!("Fayilga qiyma kiriting:> ");
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Qiymat kiritishda xatolik");
    fayl.write_all(input.as_bytes()).expect("fayildagi qiymatlarni oqishda xatolik bor")
}
