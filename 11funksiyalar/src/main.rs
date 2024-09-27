fn main() {
// funksiyalarni asosiy funksiyaga chaqirish
    hello();
    sum(8, 4); // sum uzgaruvchisiga argument berish
    info(String::from("Vali"), 24, 4);
    qiymatlar();
// malum bir tipda qaytish funksiyani asosiy funksiyaga chaqirish
    println!("");
    let c = kopaytma(4, 5);
    println!("{c}");

// math funksiyasini chaqirib olindi
    println!("");
    let (a, b, c) = math(4, 5, 6);
    println!("{a}, {b}, {c}");
}


// funksiya yaratish 
fn hello() {
    println!("Hello");
}

// elemetn qabul qilsh 
fn sum(a: i32, b: i32) { // sum o'zgaruvchisiga parametr berish
    println!("{a} + {b} = {}", a + b);
}

fn info(name:String, age: i32, kurs: i8) {
    println!("O'quvchi {name} yoshi {age} da, {kurs}-kurs talabasi");
}

fn qiymatlar() {
    let qiymat = {
        let b = 0;
        b + 1
    };
    println!("{}", qiymat)
}
// bit nechat qiymatlar qaytarish va kiritish, tiplarni qaytarish
fn kopaytma(a: i32, b: i32) -> i32 { // -> i32, qaysi tipda qaytishini korsatish
    a * b //funksiyadan qiymatni qaytarish
}

// Bir nechta qiymatlar va tiplarni qaytishi
fn math(a: i32, b: i32, c: i32,) -> (i32, i32, i32) {
    (a * b, b / c, c % b)
}