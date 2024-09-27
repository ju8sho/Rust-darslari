fn main() {
    let yosh: i32 = 27;
    if yosh >= 18 {
        println!("Yoshingiz yetarli");
    } else {
        println!("Yoshingiz yetarli emas");
    }

    let num = 8;
    if num > 10 {
        println!("num > 10");
    }else if num < 40 {
        println!("num < 40");
    }
    // yoki || , va && operatorlari

    if yosh >= 18 || yosh == 27 {println!("Qabul qilindingiz");}
    else if yosh < 18 && yosh != 27 {println!("Siz qabul qilinmadingiz");}
}

