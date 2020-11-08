use std::cmp::min;
// max dimension for a collection. 128Mib should be enought
// this is needed because fuzzing could try to allocate
// tera of memory and die of OOM.
static MAX_COLLECTIONS_SIZE: usize = 1024 * 1024;

pub trait Arbitrary: Sized {
    /// Convert the object to bytes
    /// 
    /// # Example
    /// ```rust
    /// use arbitrary_rust::Arbitrary;
    /// 
    /// #[derive(Arbitrary, Debug)]
    /// struct MyStruct {
    ///     id: u64,
    ///     name: String,
    ///     values: Vec::<f64>
    /// }
    /// 
    /// let value = MyStruct{
    ///     id: 0,
    ///     name: "Sergio".to_string(),
    ///     values: vec![1.0, 2.0, 3.0, 4.0]
    /// };
    /// 
    /// println!("the bytes are: {:?}", value.to_bytes());
    /// ```
    fn to_bytes(&self) -> Vec<u8>;
    /// Build object from bytes
    /// 
    /// # Example
    /// ```rust
    /// use arbitrary_rust::Arbitrary;
    /// 
    /// #[derive(Arbitrary, Debug)]
    /// struct MyStruct {
    ///     id: u64,
    ///     name: String,
    ///     values: Vec::<f64>
    /// }
    /// 
    /// let value = MyStruct::from_bytes(vec![0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 
    ///     0, 0, 0, 0, 0, 83, 101, 114, 103, 105, 111, 4, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 
    ///     0, 8, 64, 0, 0, 0, 0, 0, 0, 16, 64]
    /// );
    /// 
    /// println!("{:?}", value);
    /// ```
    fn from_bytes(data: Vec<u8>) -> Self {
        Self::build_from_bytes(&data[..]).0
    }
    /// Internal method, not generally usefull for the users
    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]);
}

mod primitives;
mod strings;
mod collections;
mod tuples;

pub use derive_arbitrary::*;