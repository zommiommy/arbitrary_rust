extern crate arbitrary_rust;
use arbitrary_rust::Arbitrary;

#[derive(Arbitrary, Debug, PartialEq)]
struct Nested {
    nested: Option::<u64>,
    name: String,
    
}
#[derive(Arbitrary, Debug, PartialEq)]
struct Test {
    nested: Nested,
    result: Result::<f64, bool>,
    array: Vec::<i128>
}

#[test]
fn test(){
    // unsigned
    for i in 0..255 {
        assert_eq!(i, u8::from_bytes(i.to_bytes()));
    }
    for i in 0..60_000 {
        assert_eq!(i, u16::from_bytes(i.to_bytes()));
    }
    for i in 0..100_000 {
        assert_eq!(i, u32::from_bytes(i.to_bytes()));
    }
    for i in 0..100_000 {
        assert_eq!(i, u64::from_bytes(i.to_bytes()));
    }
    for i in 0..100_000 {
        assert_eq!(i, u128::from_bytes(i.to_bytes()));
    }
    for i in 0..100_000 {
        assert_eq!(i, usize::from_bytes(i.to_bytes()));
    }

    // signed
    for i in 0..127 {
        assert_eq!(i, i8::from_bytes(i.to_bytes()));
    }
    for i in 0..30_000 {
        assert_eq!(i, i16::from_bytes(i.to_bytes()));
    }
    for i in 0..100_000 {
        assert_eq!(i, i32::from_bytes(i.to_bytes()));
    }
    for i in 0..100_000 {
        assert_eq!(i, i64::from_bytes(i.to_bytes()));
    }
    for i in 0..100_000 {
        assert_eq!(i, i128::from_bytes(i.to_bytes()));
    }
    for i in 0..100_000 {
        assert_eq!(i, isize::from_bytes(i.to_bytes()));
    }

    // floats
    for i in 0..100_000 {
        let i = i as f32;
        assert_eq!(i, f32::from_bytes(i.to_bytes()));
    }
    for i in 0..100_000 {
        let i = i as f64;
        assert_eq!(i, f64::from_bytes(i.to_bytes()));
    }

    //vec
    let v: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
    assert_eq!(
        v,
        Vec::<u8>::from_bytes(v.to_bytes())
    );
    let v: Vec<u16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
    assert_eq!(
        v,
        Vec::<u16>::from_bytes(v.to_bytes())
    );
    let v: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
    assert_eq!(
        v,
        Vec::<u32>::from_bytes(v.to_bytes())
    );
    let v: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
    assert_eq!(
        v,
        Vec::<u64>::from_bytes(v.to_bytes())
    );
    let v: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
    assert_eq!(
        v,
        Vec::<i8>::from_bytes(v.to_bytes())
    );
    let v: Vec<i16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
    assert_eq!(
        v,
        Vec::<i16>::from_bytes(v.to_bytes())
    );
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
    assert_eq!(
        v,
        Vec::<i32>::from_bytes(v.to_bytes())
    );
    let v: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]; 
    assert_eq!(
        v,
        Vec::<i64>::from_bytes(v.to_bytes())
    );
    // composition
    let v: Vec<Vec<i64>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 0]]; 
    assert_eq!(
        v,
        Vec::<Vec::<i64>>::from_bytes(v.to_bytes())
    ); 
    let v: Vec<Vec<Vec<i64>>> = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]], vec![vec![9, 0]]]; 
    assert_eq!(
        v,
        Vec::<Vec::<Vec::<i64>>>::from_bytes(v.to_bytes())
    );
    // option
    assert_eq!(
        None::<u64>,
        Option::from_bytes(None::<u64>.to_bytes())
    );
    assert_eq!(
        Some(100u64),
        Option::from_bytes(Some(100u64).to_bytes())
    );
    // String
    assert_eq!(
        "".to_string(),
        String::from_bytes("".to_string().to_bytes())
    );
    assert_eq!(
        "RAVIOLI RAVIOLI GIVE ME THE FORMULORI".to_string(),
        String::from_bytes("RAVIOLI RAVIOLI GIVE ME THE FORMULORI".to_string().to_bytes())
    );
        // Result
    assert_eq!(
        Ok(None),
        Result::<Option<u64>, String>::from_bytes(Ok::<Option<u64>, String>(None).to_bytes())
    );
    assert_eq!(
        Err::<Option<u64>, u64>(1),
        Result::from_bytes(Err::<Option<u64>, u64>(1).to_bytes())
    );
    // bool
    assert_eq!(
        true,
        bool::from_bytes(true.to_bytes())
    );
    assert_eq!(
        false,
        bool::from_bytes(false.to_bytes())
    );
    // tuples
    assert_eq!(
        (),
        <()>::from_bytes(().to_bytes())
    );
    assert_eq!(
        (true, false),
        <(bool, bool)>::from_bytes((true, false).to_bytes())
    );
    assert_eq!(
        (true, false, 1.0),
        <(bool, bool, f64)>::from_bytes((true, false, 1.0).to_bytes())
    );
    assert_eq!(
        (1,),
        <(u64,)>::from_bytes((1u64,).to_bytes())
    );
    // struct
    let test = Test{
        nested: Nested{
            nested:Some(u64::MAX),
            name: "CARDAMOMO".to_string()
        },
        result: Ok(69.420),
        array: vec![1, 2, 3, u64::MAX as i128 + 1337]
    };
    assert_eq!(
        test,
        <Test>::from_bytes(test.to_bytes())
    )
}