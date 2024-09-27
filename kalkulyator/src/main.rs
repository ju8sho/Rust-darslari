use std::io;

fn main() {

    loop {
        // Foydalanuvchidan operatsiya so'rash
        println!("Qanday amalni bajarishni xohlaysiz? ((+), (-), (*), (/), (exit))"); 

        let mut amallar = String::new();          //matin saqlash uchun bosh ozgaruvchi
        io::stdin().read_line(&mut amallar).expect("amallarni kiritishda xatolik"); // foydalanuvchidan qiymat soraymiz va bosh matinga qiymatni yozamiz
        let amallar = amallar.trim();               //matinda bosh joylarini olib tashlayniz

        if amallar =="exit" {       // agar amallar exit bolsa
            println!("dastur tuxtadi");
            break;                  //dasturni toxtatamiz
        };
    
    
        let son1 = loop {                             // sikl orqali kirivchi malumotlarni qayta ishlaymiz
            println!("Birinchi sonni kiriting: ");         //Foydalanuvchidan birinchi sonni kiritishni so'raymiz

            let mut son = String::new();            //Bo'sh satr (String) o'zgaruvchisini yaratamiz
            io::stdin().read_line(&mut son).expect("1-sonni kiritishda xatolik");  //Foydalanuvchidan kiritilgan matnni o'qib, `son1` ga saqlaymiz
            let son = son.trim();                     //Matndan ortiqcha bo'sh joylarni olib tashlaymiz
            
            match son.parse::<f64>() {                      // 5. Matnni `f64` turiga aylantirishga harakat qilamiz
                Ok(res) => break res,                  //Agar muvaffaqiyatli bo'lsa, qiymatni qaytaramiz va loopdan chiqamiz
                Err(_) => println!("Iltimos tog'ri son kiriting")  //Agar xatolik yuz bersa, foydalanuvchiga xatolik haqida xabar beramiz
            }
        };

    
        let son2 = loop {
            println!("Ikkinchi sonni kiriting:");
            
            let mut son2 = String::new();
            io::stdin().read_line(&mut son2).expect("2-sonni kiritishda xatolik");
            
            match son2.trim().parse::<f64>() {
                Ok(res) => break res,
                Err(_) => println!("To'g'ri son kiriting")          // sikl yana qayta son soraydi
            }
        };
        

    // Operatsiyaga qarab natijani hisoblash
        xisoblash(amallar, son1, son2)

    }


}


fn xisoblash(amallar: &str, son1:f64, son2:f64){
    match amallar {
        "+" => println!("{son1} + {son2} = {}", son1 + son2),
        "-" => println!("{son1} - {son2} = {}", son1 - son2),
        "*" => println!("{son1} * {son2} = {}", son1 * son2),
        "/" => {
            if son2 != 0.0 {println!("{son1} / {son2} = {}", son1 / son2)}    //son2 0 ga teng emasmi?
            else {println!("no'lga bolish mumkin emas")}
        },
        _=> println!("Nomalum amallar"),

    }
}