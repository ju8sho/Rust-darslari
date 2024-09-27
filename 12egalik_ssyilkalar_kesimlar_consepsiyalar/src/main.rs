mod input;

fn main() {
/* 
!Mulkchilik (Ownership)
!Rust tilidagi mulkchilik qoidalari xotira boshqaruvini xavfsiz va samarali qilish uchun yaratilgan. Bunda uchta asosiy qoida mavjud:

!Har bir qiymat Rust dasturida o'zining mulkdori (owner) ga ega.
!Bir vaqtning o'zida faqat bitta mulkdor bo'lishi mumkin.
!Mulkdorning doirasi (scope) tugagach, qiymat o'chiriladi (drop qilinadi).
*/ 
// Misol:

    let s1 = String::from("hello"); // s1 ni egalik huquqi s2 ga o'tadi
    let s2 = s1; // s2 s1 ning  qiymatiga egalik qiladi
    println!("s2: {} \ns1 uzgaruvchi endi xotiradan uchub ketadi", s2);
    println!(); // Qator bo'shlig'i

    // agar uzgaruvchila raqamlardan stikerlardan, bool, lardan tashkil topgan bolsa egalik ga zaruryat bolmaydi ular togridan togri kopiya qilinadi
    let x = 5;
    let y = x; // bu yerda x o'zgaruvchisini y degan o'zgaruvchiga kopiya qiladi
    println!("x:{} y:{}", x, y);


// /////////////////////////////////////////////////////////////

/*
!Ssıilkalar (References)
!Rust tilida ssıilkalar qiymatlarni mulkchilikni o'zgartirmasdan qaratish imkonini beradi. Ssıilkalar ikki xil bo'ladi: qarzga olingan (borrowed) va o'zgaruvchan (mutable).

!Qarzga olingan ssıilka (Borrowed Reference): Qiymatni o'zgartirmasdan qarzga olish uchun ishlatiladi.
!O'zgaruvchan ssıilka (Mutable Reference): Qiymatni o'zgartirish uchun ishlatiladi.
*/

    // Qarzga olingan ssıilka
    let a = String::from("Salom");
    let qancha_uzunlik  = uzunlik(&a); //a o'zgaruvchisidan qancha_uzunlik degan uzgaruvchiga qarz olinmoqda qarz olish bu & belgi bn ifodalanadi 
    println!("a ning qiymatini uzunligi: {} ga teng", qancha_uzunlik);
    println!("a:{}", a);// a o'zgaruvchisini qaytaradi

    println!(); // Qator bo'shlig'i

    // O'zgaruvchan ssıilka
    let mut b = String::from("Salom");
    matin_qoshish(&mut b);
    println!("{}", b);

//////////////////////////////////////////////////////////
    println!(); // Qator bo'shlig'i
/*Kesmlar (Slices)
Kesmlar (slices) Rust tilida massiv yoki stringning bir qismini qarzga olish imkonini beradi. Kesmlar qiymatni o'zgartirmasdan foydalanish uchun ishlatiladi. */
    let s = String::from("Salom, Dunyo");
    let salom = &s[0..5]; // s o'zgaruvchisining 0 dan 5 gacha  qiymat oraligi salom degan o'zgaruvchiga qarz olinmoqda
    let dunyo = &s[6..12]; // s o'zgaruvchisining 6 dan 12 gacha qiymatini oraligi dunyo degan o'zgaruvchiga qarz olinmoqda
    println!("{} {}", salom, dunyo);

    let array = [1, 2, 3, 4, 5];
    let tarif = &array[0..4]; // array o'zgaruvchisining 0 dan 4 gacha bolgan qiymat oraligi, tarif degan o'zgaruvchiga qarz olinmoqda
    println!("array:{:?}", tarif);


////////////////////////////////////////////////////////////////////////////////////////
/*
!Qo'shimcha Misollar va Qoidalar
!Mulkchilik va Ssıilkalar
!Mulkchilik va ssıilkalarni birgalikda ishlatishning ba'zi qoidalari:

!Bir vaqtda faqat bitta o'zgaruvchan ssıilka yoki ko'plab qarzga olingan ssıilkalar bo'lishi mumkin.
!Mulkchilikni yoki o'zgaruvchan ssıilkani qarzga olgan ssıilka yo'q bo'lsa, mulkchilikni o'zgartirish mumkin.
 */
println!(); // Qator bo'shlig'i
    // Misol:
    let mut matn = String::from("Salom");

    let s1 = &matn;
    let s2 = &matn;
    println!("s1:{}, s2:{}", s1, s2);

    
    let s3 = &mut matn;
    s3.push_str(", Rust");
    println!("matn qo'shildi:{}", matn);

    println!();
    // input::ism()

}


// ///////////////////////////////////////////
// Qo'shimcha funksiyalar
fn uzunlik(s: &String) -> usize {
    s.len()
}

fn matin_qoshish(s: &mut String) {s.push_str(", Rust")} // bu funksiya s degan qiymat qabul qiladi va push_str shu qiymatga matin qo'shadi