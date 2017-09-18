/* use std::io;
use std::result;

pub enum Error {
    IO(io::Error),
    InvalidFile,
    NYI
}

pub type Result<T> = result::Result<T, Error>;


impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
} */

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        BrokenFileFormat
        NYI
    }
}