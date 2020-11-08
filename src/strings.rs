use super::*;

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
        let (mut len, mut data) = usize::build_from_bytes(data);
        len = min(len, MAX_COLLECTIONS_SIZE);
        let mut result = String::with_capacity(len);
        for _ in 0..len {
            let (value, new_data) = char::build_from_bytes(data);
            data = new_data;
            result.push(value);
        }
        (result, data)
    }
}