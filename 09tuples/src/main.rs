fn main() {
/* (tuples) — bu bir necha turdagi qiymatlarni bitta birlikda saqlash uchun ishlatiladigan ma'lumotlar tuzilmasi. Ular ko'p hollarda statik uzunlikka ega va ular ichidagi elementlar turli xil turlarga ega bo'lishi mumkin.*/
//  tuples larni yaratish

    let tuple:(i32, f32, String, char) = (123, 2.4, String::from("Bu tuple malumotlar tuzulmasi"), '👍');
    println!("{:?}", tuple);

// qiymatlarni olish

    let uquvchilar = (String::from("Ali"), 11);
    let (name, sinf) = uquvchilar;
    println!("O'quvchi: {}, {}-sinf", name, sinf);

    println!("");

// elementlarga  indeks orqali murojat qilish
    let talabalar = (String::from("Guni"), String::from("Vali"), 2, 4);
    let talaba1 = talabalar.0;
    let talaba2 = talabalar.1;
    println!("O'quvchilar: {} {}-kurs talabasi, {} {}-kurs talabasi", talaba1, talabalar.2, talaba2, talabalar.3);

}
