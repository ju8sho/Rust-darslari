use std::error::Error;
use std::fmt::{Display, Formatter};


#[derive(Debug)]
enum CustomErr {

    Err1(String),
    Err2(i32),
    Err3(i32, String)

}


impl Display for CustomErr {
    
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {

        match self {

            Self::Err1(msg) => {
                write!(f, "Err1 xatosi {}", msg )
            },

            Self::Err2(num) => {
                write!(f, "Err2 xatosi {}", num)
            },

            Self::Err3(num, msg) => {
                write!(f, "Err3 xatosi {} {}", num, msg)
            }
        }   
    }

}


impl Error for CustomErr {}


fn xato_qaytar() -> Result<(), CustomErr> {

    // code
    Err(CustomErr::Err3(34, "Xatolar..".to_owned()))

}

fn main() {

    match xato_qaytar() {
        Ok(_) => {}
        Err(err) => println!("{}", err)
    }
    
}
