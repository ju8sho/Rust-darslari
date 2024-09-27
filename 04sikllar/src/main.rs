fn main() {
    // sikllar
    // loop
    // while
    // for

    // loop siklidan foydalanish
    let mut i = 1;
    loop {
        println!("{i}"); i += 1;
        if i == 21 {break;} // agar i 21 ga teng bo'lganda siklni toxtat. break
    }

    // while siklidan foydalanish
    let mut num = 0;
    while num <= 5 {println!("{num}"); num += 1;} // toki num 5 dan kichik ekan kodni bajar

    let mut x = 0;
    while x <= 10 {
        if x % 2 == 0 {println!("{}", x);} //juf sonlarni chiqarish
        x += 1;
    }
    
    // for sikli
    for n in 1..11 {
        println!("Bu for sikli {n}");
    }
    
    // for sikli orqali juf sonlarni chiqarish
    for y in 1..11 {
        if y % 2 == 0 {println!("{y}")}
    }
}
