const MAX: i8 = 11;

fn main() {
    const NAME: &str = "bu constant";
    const NUM: i8 = 3;
    println!("{NAME}");
    println!("{NUM}");

    println!("{}",MAX);
    println!(""); // konsolda natijalar ajralib turish uchun joy tashlandi
    // siklda ishlattish
    for i in 1..MAX {println!("{}", i)}
}
