use super::*;

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
        let (mut len, mut data) = usize::build_from_bytes(data);
        len = min(len, MAX_COLLECTIONS_SIZE / std::mem::size_of::<T>());
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