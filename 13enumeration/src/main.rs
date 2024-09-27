
// Enum nima?
// Enum (enumeratsiya) bu qiymatlar to'plamini belgilash uchun ishlatiladi. U ko'pincha ma'lum bir holatlar yoki turli variantlarni ifodalash uchun ishlatiladi. Rust tilida enumlar bir nechta variantlarga ega bo'lib, har bir variant o'z qiymatiga ega bo'lishi mumkin.
//Masalan, hafta kunlari, yo'l harakati svetofori ranglari yoki o'yin ichidagi turli holatlar kabi narsalarni enum yordamida belgilash mumkin.
// enum yaratish 
enum Svetafor {
    Qizil,
    Sariq,
    Yashil
}


fn main() {
    // enumdan foydalanish
    let svetafor_chrogi = Svetafor::Yashil;

    // variantlarni olish
    match svetafor_chrogi {
        Svetafor::Qizil => println!("Toxta"),
        Svetafor::Sariq => println!("Tayyorlaning"),
        Svetafor::Yashil => println!("Harakatlaning"),
    }
    
}
