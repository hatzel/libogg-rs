pub mod packet;
pub mod page;
pub mod stream;

pub use crate::packet::Packet;
pub use crate::page::Page;
pub use crate::stream::Stream;

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InternalError,
    TypeCastingError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::InternalError => write!(f, "Internal libogg error"),
            Error::TypeCastingError => write!(f, "Type casting error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::packet::Packet;
    use crate::stream::Stream;
    #[test]
    fn init_stream() {
        let _stream = Stream::new(32);
    }

    #[test]
    fn stream_debug() {
        let stream = Stream::new(32);
        println!("{:#?}", stream);
    }

    #[test]
    fn new_packet() {
        let mut vec = Vec::new();
        for x in 0..255 {
            vec.push(x);
        }
        let mut p = Packet::new(&mut vec);
        let mut s = Stream::new(0);
        s.packetin(&mut p);
        println!("{:#?}", p);
    }

    #[test]
    fn read_packet() {
        let mut s = Stream::new(0);
        for _ in 0..10 {
            loop {
                match s.pageout() {
                    Some(page) => {
                        println!("header: {:?}/body {:?}", page.header, page.body);
                        break;
                    }
                    None => {
                        let mut vec = Vec::new();
                        for x in 0..255 {
                            vec.push(x);
                        }
                        let mut p = Packet::new(&mut vec);
                        s.packetin(&mut p)
                    }
                }
            }
        }
        println!("{:?}", s);
    }
}
