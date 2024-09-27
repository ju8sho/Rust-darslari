fn main() {
    let list = vec![10, 20, 30, 567];

    println!("{}", arifmetika(&list));
}

fn arifmetika(l: &Vec<i32>) -> f64 {
    let mut sum = 0;

    for i in l {
        sum += i;
    }

    let resul = (l.len()) as i32;
    let sum = sum as f64;

    sum / resul as f64
}