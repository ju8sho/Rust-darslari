// Rust tilida Option<T> turi element mavjud yoki mavjud emasligini bildiruvchi turdir. Option<T> ikkita qiymatga ega bo'lishi mumkin: Some(T) yoki None.

/* Some(T) – element mavjud bo'lsa va T qiymatini o'z ichiga olsa.
None – element mavjud bo'lmasa.
get Metodi
Vec<T> turi uchun get metodi indeks bo'yicha elementni olishga imkon beradi. Ushbu metod Option<&T> ni qaytaradi. Agar indeks mavjud bo'lsa, Some(&T) ni, aks holda None ni qaytaradi. */

fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Options: Some va None 

    println!("{:?}", list.get(3)); // Agar indeks mavjud bo'lsa, Some(&T) ni qaytaradi. 
    println!("{:?}", list.get(11)); // None, 11-element mavjut emas 

    // match operatoridan foydalanish
    match list.get(9) {
       Some(el) => println!("{}-element", el),
       None => println!("None"),
    };
}