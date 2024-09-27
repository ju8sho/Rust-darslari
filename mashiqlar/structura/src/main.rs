/*1. Avto Strukturasini Yaratish
Avto klassini Rustda struct yordamida yaratamiz, unda avtomobilning turli xususiyatlari, masalan, model, rang, korobka, narx, va kilometer, kiritilgan bo'ladi.*/

struct Avto {
    model: String,
    rang: String,
    korobka:Karobka,
    narx: f64,
    kilometer: u32,
}
/*2. Enum orqali Korobka Tiplarini Ifodalash
Korobka turini enum yordamida ifodalashimiz mumkin. */
#[derive(Debug)]
enum Karobka {
    Manual,
    Automatik,
}
/*3. Avto uchun Trait Yaratish va Implementatsiya Qilish
Car strukturasiga xos xususiyatlar va metodlar uchun trait yaratamiz va uni implementatsiya qilamiz. */
trait AvtomobilHarakatlari {
    fn get_info(&self) -> String; // Avto haqida ma'lumot qaytaruvchi metod
    fn update_km(&mut self, km: u32); // Avto yurilgan kilometrajini yangilovchi metodb
}
// Traitni Car structiga implementatsiya qilish
impl AvtomobilHarakatlari for Avto {
    fn get_info(&self) -> String {
        format!("Model: {}, rangi: {}, karobka: {:?}, narxi: {}$, km: {}", self.model, self.rang, self.korobka, self.narx, self.kilometer)
    }

    fn update_km(&mut self, km: u32) {
        self.kilometer += km
    }
}

// Avtosalonni ifodalovchi struktura
struct AvtoSalon {
    name: String,
    address: String,
    avtomabillar: Vec<Avto>,
}
// avto salon metodlarini yozamiz
impl AvtoSalon {
    fn add_avto(&mut self, car: Avto) {
        self.avtomabillar.push(car)
    }
    fn get_avto_info(&self) -> String {
        self.avtomabillar.iter().map(|car| car.get_info()).collect::<Vec<String>>().join("\n")
    }
}

fn main() {
    // obyektlar yaratib olish
    let car1 = Avto {
        model: "GM".to_string(),
        rang: "Qora".to_string(),
        korobka: Karobka::Automatik,
        narx: 13.000,
        kilometer: 0,
    };
    let car2 = Avto {
        model: "Tesla".to_string(),
        rang: "Sariq".to_string(),
        korobka: Karobka::Manual,
        narx: 23.000,
        kilometer: 0,
    };
    let car3 = Avto {
        model: "Mustang".to_string(),
        rang: "Oq".to_string(),
        korobka: Karobka::Automatik,
        narx: 30.000,
        kilometer: 0,
    };
//      salonga mashinalarni qoshamiz
    let mut avto_salon = AvtoSalon {
        name: "SupperCars".to_string(),
        address: "Pop shaxar Sang qishlogi".to_string(),
        avtomabillar: vec![car1, car2],
    };
//     add_avto metodi orqali avtolar qoshamiz
    avto_salon.add_avto(car3);

    println!("{}", avto_salon.get_avto_info());
    println!();
//     Avtomobilning kilometrajini yangilaymiz
    if let Some(res) = avto_salon.avtomabillar.get_mut(0) {
        res.update_km(100) // bu yerda avtosalondagi 1 -da turgan mashinani km ozgartirdik
    }
    println!("{}", avto_salon.get_avto_info())

}
