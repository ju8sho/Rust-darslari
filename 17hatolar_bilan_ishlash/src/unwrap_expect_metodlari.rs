use std::fs::File;

fn main() {
    let path = "enwrep_expect.txt";
//  unwrap() agar fayil bolsa Result tipini qaytaradi yani fayilni qaytaradi agar fayil bolmasa u xato qaytaradi
    let f = File::open(path).unwrap(); 

    let e = File::open("expect.txt").expect("Bunday file yoq");


}