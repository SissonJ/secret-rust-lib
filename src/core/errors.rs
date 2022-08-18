use std::fmt;

type Result<T> = std::result::Result<T, Error>; 

#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    NumberError(i32),
    DecNumError(i128),
    DecStringError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NumberError(i) => write!(f, "NumberError for i32 {}", i),
            Error::DecNumError(i) => write!(f, "DecNumError for Dec{{ i: {} }}", i),
            Error::DecStringError(m) => write!(f, "DecNumError: {}", m),
        }
    }
}

