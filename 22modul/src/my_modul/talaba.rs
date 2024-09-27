#[derive(Debug)]
pub struct Talaba {

    pub ism: String,
    pub yosh: i32,
    pub fakultet: String

}

impl Talaba {

    //puplichniy konstuktor
   pub fn new(ism: String, yosh: i32, fakultet: String) -> Self {
       Self {ism, yosh, fakultet} // bu yerda  hardoyim Structuradagi qiymatlarni publicat qilish shart emas
   }
   
}