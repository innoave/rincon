
pub trait ToAql {
    fn to_aql(&self, out: &mut ToAqlOutput) -> Result<IsNull, Error>;
}

#[derive(Debug)]
pub struct ToAqlOutput {

}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IsNull {
    Yes,
    No,
}

#[derive(Debug, PartialEq)]
pub struct Error {

}
