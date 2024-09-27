fn main() {
    // bir turdagi malumotlar yigindisi (arrays) 
    let mut arr: [i32; 8] = [1, 3, 3, 4, 5, 6, 7, 8];

    // bir (har xil tipdagi) elementni birnechta qilib chiqarish
    let num = [4; 5]; // num array elementi 4 dan 5 ta konsolga chiqarish
    println!("{:?}", num );

    // arrayga yangi elementlar qoshish
    arr[1] = 2;

    // elementlarini indeks orqali olish
    println!("{:?} ", arr[1]);
    println!("{:?}", arr);

    /* 
    ! Rust'da iter() metodi, bir dilim (slice) yoki kolleksiyadagi elementlarni iterator orqali ketma-ket ko‘rib chiqish imkonini beradi. Bu metod, elementlarni o‘zgartirmasdan ulardan o‘tishga imkon beradi. Agar elementlarni o‘zgartirish kerak bo‘lsa, iter_mut() metodidan foydalaniladi.
    */
    for iter in arr.iter() {
        println!("{}", iter);
    }
    // elementlarni o'zgartirish iter_mut() metod
    /*
     ! *iter += 1;: Har bir element qiymatini 1 ga oshiradi. *iter orqali iterator tomonidan ko‘rsatilgan qiymatga murojaat qilinadi va o‘zgartiriladi. */
    for iter in arr.iter_mut() {
        *iter += 1;
        println!("{}", iter);
    }

    for iter in 0..arr.len() {println!("{}", arr[iter] )};
}
