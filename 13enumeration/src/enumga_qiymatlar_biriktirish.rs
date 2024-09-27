// Enumga qiymat biriktirish
// Ba'zi hollarda, enum variantlariga qo'shimcha ma'lumotlar biriktirish kerak bo'lishi mumkin.

enum Habar {
    Quit, // qiymatga ega emas
    Sonlar {x: i32, y: i32}, // 2ta integer qiymatlarga ega
    Oqish(String),
    Ranglar(i32, i32, i32), // 3ta integer tipga  ega
}

// Enumga qiymat biriktirish
// Ba'zi hollarda, enum variantlariga qo'shimcha ma'lumotlar biriktirish kerak bo'lishi mumkin.

fn main() {
    let msg = Habar::Sonlar {x: 30, y: 45};

    // msg ni qiymatini tekshiramiz
    match msg {
        Habar::Quit => println!("Chiqish"),
        Habar::Sonlar {x, y} => println!("Bu ikki sonlar: {}, {}", x, y),
        Habar::Oqish(matn) => println!("bu matin:{}", matn),
        Habar::Ranglar(a, b, c) => println!("Bu ranglar:{} {} {}", a, b, c),
    }
}