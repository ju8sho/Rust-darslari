use std::fmt::{Debug, Display};

fn main() {
    let son = 34;
    let matn = "Hello Rust";
    korsat(son);
    korsat(matn);
    // 
    let res = salomdunyo();
    println!("{}", res);
    // 
    let a = 4;
    let b = 6;
    taqqoslash_korsatish(a, b);

    //
    let list: Vec<i32> = vec![1,2,3,4,5,5,6,6,7,7,5];
    let takror = dublikat_topish(&list);
    println!("takrorlangan {:?}", takror);

    let belgilar = ["e", "d", "b", "b", "y"];
    let takrorlangan = dublikat_topish(&belgilar);
    println!("takrorlanganlar : {:?}", takrorlangan);
}




fn korsat<T: Display>(qiymat: T) {println!("{}", qiymat)}// Bu misolda ko'rsat funksiyasi Display xususiyatini amalga oshiradigan har qanday tur bilan ishlay oladi.

//  Ma'lum bir xususiyatni amalga oshiradigan turlarni qaytarish
fn salomdunyo() -> impl Display { // bu funksiya Display trait ni qaytaruvchi qiymatni oladi
    "Salom dunyo" // matini qaytaradi
}

fn taqqoslash_korsatish<T>(a: T, b: T) // funksiya Option T turi bolib u 2 ta qiymat oladi T turida 
where 
    T: PartialOrd + Debug, // bu qiymatlar hususiyatini korasatamiz, hususiyatlar jegarasini belgilaymiz
{
    if a > b {println!("{:?} katta {:?} dan", a, b);}
    else {println!("{:?} katta emas {:?} dan",a, b)}
}

fn dublikat_topish<T>(list: &[T]) -> Vec<T> // 2. Slice qabul qiladigan funksiyani e'lon qilamiz
where
    T: PartialEq + Copy + Debug, // 3. Xususiyatlar: tenglik, nusxalash va chop etish imkoniyatlari
{
    let mut dublikat: Vec<T> = Vec::new(); // 4. Takrorlanadigan elementlarni saqlash uchun bo'sh vektor

    for uzunlik in 0..list.len() { // 5. Tashqi sikl: 0-dan list uzunligigacha
        for res in (uzunlik + 1)..list.len() { // 6. Ichki sikl: tashqi sikldan keyingi elementdan list uzunligigacha
            if list[uzunlik] == list[res] { // 7. Agar tashqi va ichki elementlar teng bo'lsa
                if !dublikat.contains(&list[uzunlik]) { // 8. Agar element allaqachon dublikatlarda bo'lmasa
                    dublikat.push(list[uzunlik]); // 9. Elementni dublikatlarga qo'shamiz
                }
            }
        }
    }
    dublikat // 10. Takrorlanadigan elementlar ro'yxatini qaytaramiz
}
