// Vektor (yoki dinamik massiv) Rust tilida o'lchami o'zgarishi mumkin bo'lgan massiv turidir. Rust tilidagi vektorlar Vec<T> turi bilan ifodalanadi, bu yerda T vektordagi elementlarning turini bildiradi.

// Vektor yaratishning bir necha usullari mavjud:

// bosh vektor yaratish

fn main() {
    let mut numbers = Vec::new();
    numbers.push(1); // bosh vektorga element qo'shish
    numbers.push(2);    
    numbers.push(3);
    numbers.push(4);
    // numbers.pop(); // oxirgi elementni olib tashlash
    numbers.pop();// oxirgi elementni (4) olib tashlash

    // elementni uchirib tashlash
    numbers.remove(0); // 0 indexdagi (1)- elementni olib tashlash

    println!("{:?}", &numbers);

    // macro yordamida vektor yaratish
    let num = vec![1, 2, 3, 4, 5];


    // vektor elementlari bilan ishlash indeksi orqali murojat qilamiz
    let element1 = &num[0]; 
    let element2 = &num[0..3];

    println!("Vektordan olingan elementlar: {:?} {:?}", element1, element2);
    println!("Vector elementlari:{:?}", num);
    println!("Vector uzunligi:{:?}", num.len());
    println!("bu vectorni ohirgi elementi {}", &num[4])


}
