use super::*;

macro_rules! impl_primitive_type {
    ($($type:ty)+) => {
        $(
            impl Arbitrary for $type {
                fn to_bytes(&self) -> Vec<u8> {
                    self.to_le_bytes().to_vec()
                }

                fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
                    if data.len() >= std::mem::size_of::<$type>() {
                        let (value, remainder) = data.split_at(std::mem::size_of::<$type>());
                        (
                            unsafe{*(value.as_ptr() as *const $type)},
                            remainder
                        )
                    } else {
                        (0 as Self, data)
                    }
                }
            }
        )+
    };
}

impl_primitive_type!(u8 u16 u32 u64 u128 usize);
impl_primitive_type!(i8 i16 i32 i64 i128 isize);
impl_primitive_type!(f32 f64);


impl Arbitrary for bool {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut (
                if *self {
                    0xff
                } else {
                    0x00
                }
            as u8).to_bytes()
        );
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (value, data) = u8::build_from_bytes(data);
        (value == 0xff, data)
    }
}