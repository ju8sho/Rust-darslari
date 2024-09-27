#[derive(Debug)]
enum Types {
    Int(i32),
    Float(f32),
    Text(String),
    Bool(bool),
}

fn main() {
    let list = vec![
        Types::Int(4),
        Types::Float(6.5),
        Types::Text("Salom Rust".to_string()),
        Types::Bool(true),
    ];

    println!("{:?}", &list);

    match &list[0] {
        Types::Int(a) => println!("{}", a),
        Types::Float(b) => println!("{}", b),
        Types::Text(c) => println!("{}", c),
        Types::Bool(d) => println!("{}", d),
    }
}

// enum orqali bir nechat tipga ega malumotlarni victorda ishlatishimiz mumkin. Shunda victor bir tipdagi yani enum tipidagi malumotlarni ishlatadi