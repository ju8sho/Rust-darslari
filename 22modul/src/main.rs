use crate::my_modul::Talaba;

mod my_modul;
fn main() {
    //Talaba

    let talaba = my_modul::Talaba {
        ism: String::from("Vali"),
        yosh: 23,

        fakultet: "Tarix".to_string()

    };
    println!("{:?}", talaba);
    

    let talaba2 = my_modul::Talaba::new("Ali".to_string(), 22, "Adabiyot".to_string());
    println!("{:?}", talaba2);

    //Shaxs
    let shaxs1 = my_modul::Shaxs {
        shaxs: Talaba {
            ism: "Ali".to_string(),
            yosh: 28,
            fakultet: "".to_string(),
        },
        kasb: "Dasturchi".to_string(),
    };
    println!("{} {}-yoshda kasbi {}", shaxs1.shaxs.ism, shaxs1.shaxs.yosh, shaxs1.kasb)

}