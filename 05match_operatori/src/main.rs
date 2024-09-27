fn main() {
    /*
    ! agar teng bolsa
    */
    let x: i32 = 4;
    match x {
       2 => println!("agar {x}, 2 ga teng bolsa"),
       4 => println!("agar {x} 4 ga teng bolsa"),
    // agar malum bir sonlar oralig'ida tenglikni tekshirish
        5..=10 => println!("5 dan 10 gacha bolgan sonlar oralig'iga teng bolsa"),

    // aksxolda 
     _ => {println!("x boshqa qiymatga ega")},
    }


    /*
    ! match ni  uzgaruvchiga yuklab tekshirish
    */
    let num = 1;
    let son: i32 = match num {
        2 => 1, // num 2 ga teng bolsa son 1 ga teng boladi
        5 => 3, // num 5 ga teng bolsa son 3 ga teng boladi
        3..=10 => 2, // num 3 dan 10 gacha bolgan sonlar oralig'iga teng bolsa son 2 ga teng boladi
        _ => 0 // aksxolda son 0 ga teng boladi
    };
    println!("{}", son);


    /*
    ! bool, true va false
    */
    let yosh: bool = false;
    let res: String;

    // agar yosh true yoki false bolsa 
    match yosh {
        true => {res = String::from("Qabul qilindi");}
        false => {res = String::from("Qabul qilinmadi");}
    }
    println!("{}", res);
}

