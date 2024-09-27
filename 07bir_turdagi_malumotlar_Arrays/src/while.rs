fn main() {
    // let arr = [1, 2, 3, 4, 5, 6, 7, 8];

    // let mut i = 0;
    // while i < arr.len() {
    //     println!("{}", arr[i]);
    //     i +=1;
    // }
    // println!("");
    // // qoldiqsiz bolinishlar
    // for y in arr.iter() {
    //     if y % 2 ==0 {println!("{y}")}
    // }
    // println!("");

    // sikl orqali takrorlangan elementlarni korish
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 6, 7, 8,];

    let mut qaytarlgan_sonlar = false;
    let mut i = 0;
    while i < array.len() {
        let mut j = i + 1;
        while j < array.len() {
           if array[i] == array[j] {
            if !qaytarlgan_sonlar {
                println!("qaytarlgan_sonlar:");
                qaytarlgan_sonlar = true;
            }
            println!("{}", array[i]);
            break;
           }
           j += 1;
        }
        i += 1;
    }
}