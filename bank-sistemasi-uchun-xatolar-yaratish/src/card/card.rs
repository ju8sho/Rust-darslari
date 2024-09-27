use chrono::{Datelike, Utc}; //kutubxonasi vaqt va sanani boshqarish uchun ishlatiladi.
use thiserror::Error; //kutubxonasi xatoliklarni yaratishda yordam beradi.

pub struct Card {

    pub(crate) balance: f64,
    pub(crate) pin: i32,
    pub(crate) card_muddati: (i32, i32)

}

// Xatolarni elon qilish
#[derive(Debug, Error)]
pub enum CardError {
    #[error("Xatolik yuz berdi, hisobingizda ${0} bor, kiritilgan summa ${1}")]
    InvalidBalance(f64, f64),

    #[error("PIN xato")]
    InvalidPIN,

    #[error("Kartaning amal qilish mudati {0}")]
    InvalidMuddati(String)

}


impl Card {
    pub fn sotib_olish(&mut self, xarajat: f64, pin: i32) -> Result<(), CardError> {

        // Karta muddati tugaganligini tekshirish
        if self.karta_mudati_tugagan() {
            return Err(CardError::InvalidMuddati(format!("{}/{}", self.card_muddati.0, self.card_muddati.1)))

        // PIN kodi to'g'riligi tekshiriladi
        } else if self.pin != pin {
            return Err(CardError::InvalidPIN)

        // Hisobdagi balans yetarliligi tekshiriladi
        } else if self.hissob_tugagan(xarajat) {
            return  Err(CardError::InvalidBalance(self.balance, xarajat))
        }

        // Xarajatni balansdan chiqarish
        self.balance -= xarajat;
        Ok(())
    }

    // Karta muddati tugaganligini tekshirish
    fn karta_mudati_tugagan(&self) -> bool {
        let joriy_sana = Utc::now().date_naive();
        let mudati_tugagan_y = self.card_muddati;

        let mudat_yil = mudati_tugagan_y.1;
        let mudat_oy = mudati_tugagan_y.0;


        mudat_yil < joriy_sana.year() || (mudat_yil < joriy_sana.year() as i32 && mudat_oy == joriy_sana.month() as i32)
    }


    // Balans etarliligi tekshiriladi
    fn hissob_tugagan(&self, harajat: f64) -> bool {
        self.balance - harajat < 0.0 // 0.0 f64 tipi ga uzgartirildi
    }
}