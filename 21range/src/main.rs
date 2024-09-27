fn main() {
    for item in 1..=10 {
        println!("{}", item);
    };

println!();
    // range yordamida 1 dan 10 gacha bolgan vector yaratish
    let num: Vec<i32> = (1..=10).collect();
    for i in num {
        println!("{}", i)
    };

println!();//

    for i in (1..=50).step_by(2) {println!("{}", i)};

println!();

    //indeks orqali murojat
    let vec: Vec<i32> = (1..=10).collect();
    let indeks = &vec[0..5]; //0 dan 5 gacha bolgan son oraligini olish
    for i in indeks {println!("{}", i)};


    for alfbo in 'a'..='z' {
        println!("{}", alfbo)
    }
}
