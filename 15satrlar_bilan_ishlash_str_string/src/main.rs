fn main() {
// satirlar

    let s: &str = "Salom rust";  // o'zgarmas matin unmutablehh

    let text = String::from("Hello World");
    let teks = "bu ham string matin.to_string() metodi".to_string();

    println!("{s} bu uzgarmas satir (&str), \n{text}, \n{teks}");

//  matinlar qoshish
    let mut matn = "Bu matin qoshildi -> ".to_string();
    // push_str() metodi
    matn.push_str("push_str() metodi ");

    // push metodi  bitta tipdagi malumot turini oladi char tipi
    let ch = ')';
    let ssilka = "Salom ";
    matn.push_str(ssilka);

    println!("{}, \nsimvol:{}", matn, ch);

println!(); // bush qator

//  Matinlarni bir biriga qoshish
    let matn1 = String::from("Salom");
    let matn2 = " Rust".to_string();
    let matn3 = " LoL".to_string();

    let res = matn1 + &matn2 + &matn3 + " Text";
    println!("{}", res);

println!();

//  Macro komanda orqali matnlarni qoshish
    let text1 = String::from("Salom");
    let text2 = "Rust".to_string();
    let text3 = "LoL".to_string();

    let format = format!("{text1} {text2} {text3} Wav bu format zur ekan",);
    println!("{}", format);

//  rust dasturlash tilida matinlarni indeksi orqali korish mumkin emas shu sabab matinlar oraligini kesib olib korish mumkin
println!(); //
    let uzunlik = format.len();

    println!("{uzunlik}");
    println!("{}", &format[..37]);

println!();

//  Matinlarni tegma teg chiqarish chars() metodi yordamida
    for sh in format.chars() {println!("{}", sh);}

println!();//

//  Baytlarini chiqarish bytes() metodi yordamida
    for byt in format.bytes() {println!("{}", byt);}

}
