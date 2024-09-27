enum SalomHabar {
    Salom(String),
    Hayir(String),
    Hatirlitun(String)
}

fn main() {
    let sal = SalomHabar::Salom("Assalomu alaykum".to_string());

    match sal {
        SalomHabar::Salom(a) => println!("{}",a),
        SalomHabar::Hayir(b) => println!("{}",b),
        SalomHabar::Hatirlitun(s) => println!("{}",s),
    }
}