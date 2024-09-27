enum Matematika { //Matematika enumi elon qilindi va maydonlariham maydonlar 2ta tipga  ega
    Qoshish(f64, f64),
    Ayirish(f64, f64),
    Kopaytirish(f64, f64),
    Bolish(f64, f64)
}

fn main() {
    let qiymat = Matematika::Qoshish(2.0, 3.0);
    
    // funksiyani chaqirib olamiz
    let result = matem_operator(&qiymat);

    println!("Result: {}", result);
}

// quymatlarni qaytaramiz qiymatlarni qaytarish uchun funksiya yaratamiz
fn matem_operator(ssilka: &Matematika) -> f64 {
    match ssilka {
        &Matematika::Qoshish(a, b) => a + b,
        &Matematika::Ayirish(a, b) => a - b,
        &Matematika::Kopaytirish(a, b) => a * b,
        &Matematika::Bolish(a, b) => a / b
    }
}