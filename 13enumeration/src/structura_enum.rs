// Enum bilan metodlarni ishlatish
// Enum variantlari uchun metodlarni aniqlash orqali, enumning ichki logikasini qulayroq boshqarish mumkin.

// enum va struct bilan ishlash

enum IpAddreslar {
    Ipv4,
    IPv6
}

struct IpAddres {
    iplar: IpAddreslar, //enum turi 
    address: String
}


// Enum uchun metodlar yozish

impl IpAddres {
    /*Bu yerda IpAddres strukturasi uchun display metodini yaratdik. Bu metod iplar maydonini tekshiradi va address maydoni qiymatini chiqaradi. */
    fn display(&self) {
        match self.iplar {
            IpAddreslar::Ipv4 => println!("Ipv4 manzili: {}", self.address),
            IpAddreslar::IPv6 => println!("IPv6 manzili: {}", self.address)
        }
    }
}

// Metoddan foydalanish
fn main() {
    // obyekitini yaratib olamiz
    let uy = IpAddres { //obyekt yaratildi
        // va qiymatlar berildi
        iplar: IpAddreslar::Ipv4, 
        address: String::from("127.0.0.1"),
    };

    let oqaga_qaytish = IpAddres {
        iplar: IpAddreslar::IPv6,
        address: String::from("::1"),
    };

    // obyektlarni chaqirib olish
    uy.display(); // display metodidan foydalanishS
    oqaga_qaytish.display();

}
