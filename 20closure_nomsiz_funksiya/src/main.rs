use std::thread;

fn main() {
    let n = 10;
    let zamikaniya = |x: i32| x + n;
    println!("{:?}", zamikaniya(3));

    let add = |x, y| x + y;
    println!("natija : {}", add(2, 4));

    let pustoy = || {println!("hech qanday paramatir olmaydi");};
    pustoy(); //chaqirib olish

    let text = String::from("Hello");
    let get_info = thread::spawn(move || {println!("{}", text)});
    get_info.join().unwrap();

    let kopaytirivchi = |a: i32| move |b: i32| a * b;
    let duble = kopaytirivchi(4);
    println!("javob: {}", duble(2));
    let kopaytr = kopaytirivchi(5);
    println!("{}", kopaytr(5));


    let mut son: i32 = 0;
    let mut oshirish = || {
        son += 1;
        println!("sonning qiymati {} ga teng", son);
    };
    // hargal chaqirilganda son qiymatini 1 ga oshiradi
    oshirish();
    oshirish();

    let s_bolen = |a: i32| a % 2 ==0; // mantiqiy qiymat qaytaradi true false
    println!("{}", s_bolen(7));


    let vector = vec![1, 2, 3, 4, 5];
    let res = || {
        for i in vector {println!("{}", i)}
    };
    res();
    
}
