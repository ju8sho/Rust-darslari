
fn main() {
    let talaba1 = Talaba {
        ism: "Ali".to_string(),
        familya: "Vali".to_string(),
        passport: "AA2346679".to_string(),
        t_yil: 1997,
        id: 445568965,
        bosqich: 1,
        jinsi: Jinsi::Erkak
    };
    let mut talaba2 = Talaba {
        ism: "Jhon".to_string(),
        familya: "Varlok".to_string(),
        passport: "BB2345309".to_string(),
        t_yil: 2000,
        id: 345456756,
        bosqich: 1,
        jinsi: Jinsi::Erkak
    };
    let shaxs1 = Shaxs {
        ism: "Hasan".to_string(),
        familya: "Valiyev".to_string(),
        t_yil: 1999,
        id: 203997554,
        jinsi: Jinsi::Erkak,
    };
    let shaxs2 = Shaxs {
        ism: "Malomat".to_string(),
        familya: "Olimova".to_string(),
        t_yil: 2004,
        id: 345675322,
        jinsi: Jinsi::Ayol,
    };

    // Talaba
    talaba2.add_bosqich(4);
    println!("{}",talaba1.get_info());
    println!("{}", talaba2.get_info());
    println!("talabaning ismi - {} {}-yoshda ",talaba1.ism, talaba1.get_yosh());
println!();//

    //Shaxs
    println!("Ism: {} jinsi: {}", shaxs2.ism, shaxs2.get_jinsi());
    println!("{}", shaxs2.get_info());
println!();//

    //Trait parametr sifatida uzatilgan funksiya cahqirish
    talaba_info(shaxs1);

println!();//

    // bir nechta traitlarni birlashtiruvchi funksiya
    // let id = talaba1.id;
    idlar(talaba1);
    idlar(shaxs2)
}

struct Shaxs {
    ism: String,
    familya: String,
    t_yil: i16,
    id: i32,
    jinsi: Jinsi
}


struct Talaba {
    ism: String,
    familya: String,
    passport:String,
    t_yil: i16,
    id: i32,
    bosqich: i8,
    jinsi: Jinsi
}

#[derive(Debug)]
enum Jinsi {
    Erkak,
    Ayol,
}

trait MerosModul {
    fn get_info(&self) -> String;
    fn get_id(&self) -> i32;
    fn get_yosh(&self) -> i16;
    fn get_jinsi(&self) -> String;
}

trait ID {}

impl ID for Talaba {}
impl ID for Shaxs {}

impl MerosModul for Talaba {
    fn get_info(&self) -> String {
        format!("{} {} {:?}-bosqich {}-yil, ID: {} passport: {}" , self.ism, self.familya, self.bosqich, self.t_yil, self.id, self.passport)
    }
    fn get_id(&self) -> i32 {self.id}
    fn get_yosh(&self) -> i16 {2024 - self.t_yil}
    fn get_jinsi(&self) -> String {
        match self.jinsi {
            Jinsi::Erkak => "Erkak".to_string(),
            Jinsi::Ayol => "Ayol".to_string(),
        }
    }
}
impl Talaba {
    fn add_bosqich(&mut self, x: i8) { self.bosqich = x }
}

impl MerosModul for Shaxs {
    fn get_info(&self) -> String {format!("{} {} {}-yil, ID: {}", self.ism, self.familya, self.t_yil, self.id)}

    fn get_id(&self) -> i32 {self.id}

    fn get_yosh(&self) -> i16 {2024 - self.t_yil}
    fn get_jinsi(&self) -> String {
        match self.jinsi {
            Jinsi::Erkak => "Erkak".to_string(),
            Jinsi::Ayol => "Ayol".to_string(),
        }
    }
}

// traitlarni funksiyalarga parametir sfatida berish
fn talaba_info<T: MerosModul>(obj: T) {println!("{}", obj.get_info())} //Trait parametr sifatida uzatilgan funksiya

// bir nechta traitlarni birlashtiruvchi funksiya
fn idlar<T>(obj:T)
    where T: ID + MerosModul
{
    println!("ID: {}",obj.get_id());
}



