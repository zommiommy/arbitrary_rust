use super::*;

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
