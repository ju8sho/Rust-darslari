/*
    bank kartasi xatolarni ishlash
    ! invalid balans balans yetishmovchilik,
    ! notug'ri pinkod,
    ! karta muddati 
*/
mod card;
use card::card::Card;

fn main() {

    let mut card = Card {
        balance: 1000.0,
        pin: 8588,
        card_muddati: (8, 2023),
    };

    match card.sotib_olish(500.0, 8588) {
        Ok(_) => println!("Muvaffaqiyatli amalga oshirildi"),
        Err(err) => println!("{err}")
    }
}
