#[derive(Debug)]

/*Strukturalar (structures) Rust tilida ma'lumotlarni saqlash uchun ishlatiladigan asosiy tuzilmalardan biridir. Ular bir nechta turdagi qiymatlarni bitta ob'ektga birlashtirish imkonini beradi. */
//Strukturani yaratish:
struct Talaba {
    ism: String,
    fakultet: String,
    kurs: u8,
    active: bool
}

#[derive(Debug)]
struct Tortburchak {
    kenglik: u32,
    balandlik: u32,
}

// tuple yordamida strukturani yaratish:

#[derive(Debug)]
struct Tuple(String, u8, bool, char, f32);

// //////////////////////////////////////////////////////////////////////////

// Strukturalarni hususiyatlari va metodlarini yaratish
/*Strukturaga ega funksiyalar
Struktura metodlari strukturaning o'zi bilan bog'liq funktsiyalarni e'lon qilish imkonini beradi. Bu funksiya impl (implementatsiya) blokida joylashadi:*/

impl Talaba {
    fn new(ism: String, fakultet: String) -> Talaba {
        Talaba {
            ism,
            fakultet,
            kurs: 1,
            active: true
        }
    }
    fn add_kurs(&mut self) {
        self.kurs +=1
    }
}

impl Tortburchak {
    fn maydon_yuzasi(&self) -> u32 {
        // metodi to'rtburchakning yuzasini hisoblaydi.
      self.kenglik * self.balandlik  
    }

    fn sigdira_oladimi(&self, boshqa: &Tortburchak,) -> bool {
        // metodi joriy to'rtburchak boshqa bir to'rtburchakni sig'dira olish-olmasligini tekshiradi.
        self.kenglik > boshqa.kenglik && self.balandlik > boshqa.balandlik
    }

    fn kvadrat(size: u32) -> Tortburchak {
        // Bu yordamchi metod kvadrat shaklidagi Turtburchak yaratadi, bunda kenglik va balandlik bir xil boladi (size).self qabul qilmaydi
        Tortburchak {kenglik: size, balandlik: size}
    }
}



// ////////////////////////////////////////////////////////////////////////////////////

/*Strukturani yaratish va undan foydalanish
Strukturani yaratish uchun strukturani chaqirishda barcha maydonlar uchun qiymatlarni taqdim etish kerak:*/
fn main() {
/* Strukturaga egalik
Rust tilida struktura maydonlari String kabi qiymatlar bo'lishi mumkin, bu esa ular ma'lumotlarga egalik qiladi. Strukturani boshqa funksiya yoki struktura ichida qayta ishlash uchun move yoki borrow qilish mumkin, o'zgaruvchilar orqali yangi maydon yaratib structuraga egalik qilish mumkin: */
// Misollar:
    let name = "Ali".to_string();

// Strukturadan obyekt yaratish:
    let talaba1 = Talaba {
        ism: name, // bu yerda name o'zgaruvchisi struktura maydoniga egalik qiladi
        fakultet: String::from("TATU"),
        kurs: 3,
        active: true
    };
// Obyektlarni malum qismini boshqa obyektlarga meros qilib olish:
    let talaba2 = Talaba {
        ism: String::from("Dilshod"),
        fakultet: String::from("NAMP"),
        kurs: 2,
        ..talaba1 // bu yerda talaba1 dan qolgan maydonlarni meros qilib oladi
    };

println!(); // bush qator

// tuple tipidagi strukturani chaqirib olish:
    let tuple = Tuple("Vali".to_string(), 3, true, '👍', 23.00);

    // Har bir maydonni o'qish:
    println!("String field: {}", tuple.0);
    println!("u8 field: {}", tuple.1);
    println!("bool field: {}", tuple.2);
    println!("char field: {}", tuple.3);
    println!("f32 field: {}", tuple.4);

    // barchasini chiqarish:
    println!("Tuple: {:?}", tuple);


// struktura metodlaridan foydalanamiz
println!(); // bush qator
    let mut talaba3 = Talaba::new(String::from("Guni"), String::from("Matematika"));
    talaba3.add_kurs();
    println!("talaba3:{} {} {} {}", talaba3.ism, talaba3.fakultet, talaba3.kurs, talaba3.active);

println!(); // bush qator
    println!("tortburchak shakilantiamiz");

    let tb1 = Tortburchak {kenglik: 4, balandlik: 3};
    let tb2 = Tortburchak {kenglik: 3, balandlik: 2};
    let tb3 = Tortburchak {kenglik: 50, balandlik: 60};

    println!("Tog'ri to'rtburchakning maydon yuzasi: {}", tb1.maydon_yuzasi());
    println!("tb1 tb2 sig'dira oladimi? {}", tb1.sigdira_oladimi(&tb2));
    
    let result = Tortburchak::sigdira_oladimi(&tb1, &tb2);
    if result == true {println!("Tog'ri to'rtburchak birbiriga sigadi");}
    else {println!("Tog'ri to'rtburchak birbiriga sigmaydi");}

    println!("tb1 tb2 sig'dira oladimi? {}", result);

    let kvadrat_shakli = Tortburchak::kvadrat(10);
    println!("kvadrat: {:?}", kvadrat_shakli);

    let tb_yuzasi = kvadrat_shakli.maydon_yuzasi();
    println!("kvadratning maydon yuzasi: {}", tb_yuzasi);



// //////////////////////////////////////////////////////////////////////

println!(); // bush qator

    //Strukturalarni konsolga chiqarish:
    println!("talaba1:{} {} {} {}", talaba1.ism, talaba1.fakultet, talaba1.kurs, talaba1.active);
    println!("talaba2:{} {} {} {}", talaba2.ism, talaba2.fakultet, talaba2.kurs, talaba2.active);

println!(); // bush qator

    println!("Talaba1:{:#?}", talaba1);
    println!("Talaba2:{:#?}", talaba2);


}
