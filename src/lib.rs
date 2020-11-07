trait Arbitrary: Sized {
    fn to_bytes(&self) -> Vec<u8>;
    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]);
    fn from_bytes(data: &[u8]) -> Self {
        Self::build_from_bytes(data).0
    }
}

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

impl Arbitrary for char {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut (*self as u8).to_bytes()
        );
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (value, data) = u8::build_from_bytes(data);
        (value as char, data)
    }
}

impl Arbitrary for String {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut self.len().to_bytes()
        );
        for c in self.chars() {
            result.append(
                &mut c.to_bytes()
            );
        }
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (len, mut data) = usize::build_from_bytes(data);
        let mut result = String::with_capacity(len);
        for _ in 0..len {
            let (value, new_data) = char::build_from_bytes(data);
            data = new_data;
            result.push(value);
        }
        (result, data)
    }
}

impl<T: Arbitrary> Arbitrary for Vec<T> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut self.len().to_bytes()
        );
        for v in self.iter() {
            result.append(
                &mut v.to_bytes()
            );
        }
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (len, mut data) = usize::build_from_bytes(data);
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            let (value, new_data) = T::build_from_bytes(data);
            data = new_data;
            result.push(
                value
            );
        }
        (result, data)
    }
}

impl<T: Arbitrary> Arbitrary for Option<T> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        match self {
            Some(val) => {
                result.push(0xffu8);
                result.append(&mut val.to_bytes())
            }
            None => {
                result.push(0x00u8);
            }
        }
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (t, data) = u8::build_from_bytes(data);
        
        if t == 0x00u8 {
            return (None, data);
        }

        let (value, data) = T::build_from_bytes(data);
        (Some(value), data)
    }
}

impl<T: Arbitrary, K: Arbitrary> Arbitrary for Result<T, K> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        match self {
            Ok(val) => {
                result.push(0xffu8);
                result.append(&mut val.to_bytes())
            }
            Err(val) => {
                result.push(0x00u8);
                result.append(&mut val.to_bytes())
            }
        }
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (t, data) = u8::build_from_bytes(data);
        
        if t == 0x00u8 {
            let (value, data) = K::build_from_bytes(data);
            (Err(value), data)
        } else {
            let (value, data) = T::build_from_bytes(data);
            (Ok(value), data)
        }

    }
}

impl Arbitrary for () {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        ((), data)
    }
}


impl<T0: Arbitrary> Arbitrary for (T0,) {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut self.0.to_bytes()
        );
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (value0, data) = T0::build_from_bytes(data);        
        ((value0,), data)
    }
}

impl<T0: Arbitrary, T1: Arbitrary> Arbitrary for (T0, T1) {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut self.0.to_bytes()
        );
        result.append(
            &mut self.1.to_bytes()
        );
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (value0, data) = T0::build_from_bytes(data);
        let (value1, data) = T1::build_from_bytes(data);
        ((value0, value1), data)
    }
}

impl<T0: Arbitrary, T1: Arbitrary, T2: Arbitrary> Arbitrary for (T0, T1, T2) {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut self.0.to_bytes()
        );
        result.append(
            &mut self.1.to_bytes()
        );
        result.append(
            &mut self.2.to_bytes()
        );
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (value0, data) = T0::build_from_bytes(data);
        let (value1, data) = T1::build_from_bytes(data);
        let (value2, data) = T2::build_from_bytes(data);
        ((value0, value1, value2), data)
    }
}

impl<T0: Arbitrary, T1: Arbitrary, T2: Arbitrary, T3: Arbitrary> Arbitrary for (T0, T1, T2, T3) {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut self.0.to_bytes()
        );
        result.append(
            &mut self.1.to_bytes()
        );
        result.append(
            &mut self.2.to_bytes()
        );
        result.append(
            &mut self.3.to_bytes()
        );
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (value0, data) = T0::build_from_bytes(data);
        let (value1, data) = T1::build_from_bytes(data);
        let (value2, data) = T2::build_from_bytes(data);
        let (value3, data) = T3::build_from_bytes(data);
        ((value0, value1, value2, value3), data)
    }
}

impl<T0: Arbitrary, T1: Arbitrary, T2: Arbitrary, T3: Arbitrary, T4: Arbitrary> Arbitrary for (T0, T1, T2, T3, T4) {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut self.0.to_bytes()
        );
        result.append(
            &mut self.1.to_bytes()
        );
        result.append(
            &mut self.2.to_bytes()
        );
        result.append(
            &mut self.3.to_bytes()
        );
        result.append(
            &mut self.4.to_bytes()
        );
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]) {
        let (value0, data) = T0::build_from_bytes(data);
        let (value1, data) = T1::build_from_bytes(data);
        let (value2, data) = T2::build_from_bytes(data);
        let (value3, data) = T3::build_from_bytes(data);
        let (value4, data) = T4::build_from_bytes(data);
        ((value0, value1, value2, value3, value4), data)
    }
}


mod test {
    use super::Arbitrary;
    #[test]
    fn test(){
        // unsigned
        for i in 0..255 {
            assert_eq!(i, u8::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..60_000 {
            assert_eq!(i, u16::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..100_000 {
            assert_eq!(i, u32::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..100_000 {
            assert_eq!(i, u64::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..100_000 {
            assert_eq!(i, u128::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..100_000 {
            assert_eq!(i, usize::from_bytes(&i.to_bytes()[..]));
        }

        // signed
        for i in 0..127 {
            assert_eq!(i, i8::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..30_000 {
            assert_eq!(i, i16::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..100_000 {
            assert_eq!(i, i32::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..100_000 {
            assert_eq!(i, i64::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..100_000 {
            assert_eq!(i, i128::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..100_000 {
            assert_eq!(i, isize::from_bytes(&i.to_bytes()[..]));
        }

        // floats
        for i in 0..100_000 {
            let i = i as f32;
            assert_eq!(i, f32::from_bytes(&i.to_bytes()[..]));
        }
        for i in 0..100_000 {
            let i = i as f64;
            assert_eq!(i, f64::from_bytes(&i.to_bytes()[..]));
        }

        //vec
        let v: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
        assert_eq!(
            v,
            Vec::<u8>::from_bytes(&v.to_bytes()[..])
        );
        let v: Vec<u16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
        assert_eq!(
            v,
            Vec::<u16>::from_bytes(&v.to_bytes()[..])
        );
        let v: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
        assert_eq!(
            v,
            Vec::<u32>::from_bytes(&v.to_bytes()[..])
        );
        let v: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
        assert_eq!(
            v,
            Vec::<u64>::from_bytes(&v.to_bytes()[..])
        );
        let v: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
        assert_eq!(
            v,
            Vec::<i8>::from_bytes(&v.to_bytes()[..])
        );
        let v: Vec<i16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
        assert_eq!(
            v,
            Vec::<i16>::from_bytes(&v.to_bytes()[..])
        );
        let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
        assert_eq!(
            v,
            Vec::<i32>::from_bytes(&v.to_bytes()[..])
        );
        let v: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
        assert_eq!(
            v,
            Vec::<i64>::from_bytes(&v.to_bytes()[..])
        );
        // composition
        let v: Vec<Vec<i64>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 0]]; 
        assert_eq!(
            v,
            Vec::<Vec::<i64>>::from_bytes(&v.to_bytes()[..])
        ); 
        let v: Vec<Vec<Vec<i64>>> = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]], vec![vec![9, 0]]]; 
        assert_eq!(
            v,
            Vec::<Vec::<Vec::<i64>>>::from_bytes(&v.to_bytes()[..])
        );
        // option
        assert_eq!(
            None::<u64>,
            Option::from_bytes(&None::<u64>.to_bytes()[..])
        );
        assert_eq!(
            Some(100u64),
            Option::from_bytes(&Some(100u64).to_bytes()[..])
        );
        // String
        assert_eq!(
            "".to_string(),
            String::from_bytes(&"".to_string().to_bytes()[..])
        );
        assert_eq!(
            "RAVIOLI RAVIOLI GIVE ME THE FORMULORI".to_string(),
            String::from_bytes(&"RAVIOLI RAVIOLI GIVE ME THE FORMULORI".to_string().to_bytes()[..])
        );
         // Result
        assert_eq!(
            Ok(None),
            Result::<Option<u64>, String>::from_bytes(&Ok::<Option<u64>, String>(None).to_bytes()[..])
        );
        assert_eq!(
            Err::<Option<u64>, u64>(1),
            Result::from_bytes(&Err::<Option<u64>, u64>(1).to_bytes()[..])
        );
        // bool
        assert_eq!(
            true,
            bool::from_bytes(&true.to_bytes()[..])
        );
        assert_eq!(
            false,
            bool::from_bytes(&false.to_bytes()[..])
        );
        // tuples
        assert_eq!(
            (),
            <()>::from_bytes(&().to_bytes()[..])
        );
        assert_eq!(
            (true, false),
            <(bool, bool)>::from_bytes(&(true, false).to_bytes()[..])
        );
        assert_eq!(
            (true, false, 1.0),
            <(bool, bool, f64)>::from_bytes(&(true, false, 1.0).to_bytes()[..])
        );
        assert_eq!(
            (1,),
            <(u64,)>::from_bytes(&(1u64,).to_bytes()[..])
        );
    }
}