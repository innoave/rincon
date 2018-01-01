
use std::io::Write;

pub trait ToAql {
    fn to_aql<W>(&self, out: &mut ToAqlOutput<W>) -> Result<IsNull, Error>
        where W: Write;
}

#[derive(Debug)]
pub struct ToAqlOutput<T> {
    out: T
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IsNull {
    Yes,
    No,
}

#[derive(Debug, PartialEq)]
pub struct Error {

}
