use std::fmt::Display;
struct Sonlar<T>{x:T, y:T} // bu yerda Sonlar structurasi harqanday tipdagi 2 ta qiymatlarni oladi 
enum Option<T> {Some(T), None}// bu yerda enum harqanday tipdagi qiymatni kiritish mumkin


fn main() {
    // structlar
    let butun_sonlar = Sonlar {x:2, y:10};
    let float_sonlar = Sonlar {x:2.5, y:2.0};

    // enumlar
    let some_num = Option::Some(5);
    let no_number: Option<i32> = Option::None;


    // funksiyalar
    chop_et_(35);
    chop_et_("Rust");
}



//funksiyalar
//buyerda T harqanday tipni qabul qiladi!! birsafar intejer bolsa keyingi safar string yoki boshq tipdagi qiymatlarni kiritish imkoni boladi
fn chop_et_<T: Display>(qiymat:T) {println!("{}", qiymat);}

/*
Oboblashlar (Generics) - Bu nima va nima uchun kerak?
Oboblashlar (generics) Rust tilida turli xil ma'lumot turlari bilan ishlash imkonini beruvchi kuchli vositadir. Bu orqali kodni umumiyroq va qayta foydalanishga moslashuvchan qilish mumkin. Oboblashlar yordamida funksiyalar, strukturalar, enumlar va traitlar turli xil ma'lumot turlari bilan ishlashi mumkin.
*/