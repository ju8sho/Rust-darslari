use std::collections::HashMap;

fn main() {

    let mut my_map = HashMap::new();//o'zgaruvchan bush lug'at yaratildi
    my_map.insert("kalit".to_string(), 12); // insert() metodi orqali lug'atga kalit va qiymat qoshish
    my_map.insert("Olma".to_string(), 34);

    println!("{:?}", my_map);

// my_map qiymatidagi "Olma" kaliti bo'lsa Some(x) ga shu kalitni o'zgartirish uchun get_mut() metodi kalitni oladi, get_mut metodi "Olma" kalitiga tegishli qiymatni qaytaradi va uni o'zgartirish uchun Some ostidagi x uzgaruvchiga o'raydi. Agar kalit topilmasa, None qaytaradi. 

    if let Some(x) = my_map.get_mut("Olma") {
        *x += 10; {println!("Olma qiymati 34 + 10 = {}", x)};// (*x) - ya'ni my_map dagi "Olma" kalitining qiymatini bildiradi va shu qiymatga 10 qo'shadi
    }

// Egalik huquqi

    let mut mevalar = HashMap::new();
    let n = "Behi".to_string();
    let m = "Nok".to_string();

    mevalar.insert(&n, 23);
    mevalar.insert(&m, 5);

    println!("{:?}", mevalar);
    println!("{}", mevalar[&m]);
    println!("{}",n);

println!();//

///////////////////////////////////////////////////////////////////////////////////////////////

// Hashmap yozish
/*vec! yordamida juftliklar ro'yxati yaratiladi.
vec.into_iter() yordamida vec dan iterator hosil qilinadi.
collect yordamida bu iterator HashMap ga yig'iladi.*/

    let mut bozorlik: HashMap<&str, i32> = vec![
        ("Non", 3),
        ("Choy", 1),
        ("Qovun", 1),
        ("Qaymoq", 2),
        ("Saryoq", 4),
    ].into_iter().collect();

    println!("{:?}", &bozorlik);

    // Elementlarni olish
    // "Non" kalitiga mos keladigan qiymatni olishga harakat qilamiz
    match bozorlik.get("Non") {
        Some(x) => println!("Non {} ta bor", x),//Agar qiymat mavjud bo'lsa
        None => println!("Element topilmadi"),//agar qiymat mavjut bolmasa
        
    }
println!(); //
//////////////////////////////////////////////////////////////////////////////////////////////////
// for sikli orqali
    for (kalit, qiymat) in &bozorlik {println!("{} {}", kalit, qiymat)}

//////////////////////////////////////////////////////////////////////////////////////////////////

// entry va to_insert() metodlari element olish yoki qoshish 
    let entry = bozorlik.entry("Choy").or_insert(1); //"Olma" kalitiga kirish yoki yangi kalit qo'shish
    *entry += 1; //agar Choy kaliti mavjut bolsa shu kalitning qiymatiga 1 ni qoshamiz
    println!("{:?}", bozorlik);

    // "Anor" kalitiga kirish yoki yangi kalit qo'shish
    let entry1 = bozorlik.entry("Anor").or_insert(0);
    *entry1 += 3; // agar kalit mavjut bolmasa yangi kalit qoshilib qiymatni 3ga 0shiramiz

    println!("{:?}", bozorlik);
//////////////////////////////////////////////////////////////////////////////////////////////
println!();//
// Matinda nechimarta harbit so'z qaytarilganini aniqlab lugat korinishida chiqaramiz.)) split_whitespace() metodi yordamida
    let matn = "Salom Salom men Rust dasturlash tiliman";

    let mut lugat = HashMap::new();

    for res in matn.split_whitespace() {
        let m = lugat.entry(res).or_insert(0);
        *m +=1;
    }
    println!("{:?}", lugat)  

}