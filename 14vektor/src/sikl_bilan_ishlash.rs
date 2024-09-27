fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in &list {
        if i % 2 == 0 {println!("{}", i)}
    };

    // iter() metodidan foydalanish bu metod ishlatilganda appersant(&) belgisini  yani ssilka qoyish shartmas
    let list2 = vec![1, 2, 3, 4, 5, 6, 7, 8];

    for itm in list2.iter() {
        println!("{:?}", itm);
    };
println!("------------------"); //bush qator

/* vektor.iter().enumerate() – indekslar va qiymatlar bo'yicha iteratsiya.
for (indeks, qiymat) – indeks va qiymatni birga olish.
println!("Indeks: {}, Qiymat: {}", indeks, qiymat) – har bir indeks va qiymatni chiqarish.*/

    let list3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for (kalit, qiymat) in list3.iter().enumerate() {
        // println!("{:?}", (kalit, qiymat));
        println!("{} {}", kalit, qiymat);
    }


}