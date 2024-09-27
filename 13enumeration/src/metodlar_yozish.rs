enum Matematika {
    Qoshish(f64, f64),
    Ayirish(f64, f64),
    Kopaytirish(f64, f64),
    Bolish(f64, f64)
}

impl Matematika {
    fn matem_operator(&self) -> f64 {
        match self {
            &Matematika::Qoshish(a, b) => a + b,
            &Matematika::Ayirish(a, b) => a - b,
            &Matematika::Kopaytirish(a, b) => a * b,
            &Matematika::Bolish(a, b) => a / b
        }
    }
}

fn main() {
    let qiymat = Matematika::Ayirish(7.0, 3.0);
    // enum metodidan foydalanish
    let result = qiymat.matem_operator();

    println!("Result: {}", result);
}