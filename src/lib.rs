pub trait Arbitrary: Sized {
    fn to_bytes(&self) -> Vec<u8>;
    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]);
    fn from_bytes(data: &[u8]) -> Self {
        Self::build_from_bytes(data).0
    }
}

mod primitives;
mod strings;
mod collections;
mod tuples;