use crate::casing::Casing;

#[derive(Debug)]
pub struct Record {
    pub pos: usize,
    pub len: usize,
    pub case: Casing,
}
