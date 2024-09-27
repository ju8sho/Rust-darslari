use thiserror::Error;

#[derive(Debug, Error)]
enum CustomErr {

    #[error("bu xato err1 {0}")]
    Err1(i32),

    #[error("bu xato err2 oddiy xato")]
    Err2,

    #[error("bu xato err3 {0} {1}")]
    Err3(i32, String)

}


fn do_something() -> Result<(), CustomErr> {
    Err(CustomErr::Err3(32, "bu text".to_owned()))
}

fn main() {

    if let Err(err) = do_something() {
        println!("{}", err)
    }
    
}
