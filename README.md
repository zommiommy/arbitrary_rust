# arbitrary_rust
My implementation of [Arbitrary](https://docs.rs/arbitrary/0.4.7/arbitrary/), 

# WHY? Just use that crate omg
I'd like but [**for some reasons**](https://github.com/rust-fuzz/arbitrary/issues/44) the original crate is not bi-directional.
So there is no good way to create a good corpus for structure-aware fuzzing nor to debug crashes (libfuzz **sometimes** does this but it's not relayable).

For this reason in this crate I implemented the Arbitrary trait as:
```rust
pub trait Arbitrary: Sized {
    // value to bytes
    fn to_bytes(&self) -> Vec<u8>;
    // bytes to value
    fn from_bytes(data: &[u8]);
    // bytes to value and bytes not used
    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]);
}
```
It supports `bool, f32, f64, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, Vec, Option, Result, char, String`
and tuples from 0 to 5 elements, and of-course nesting of any of these types.
In the future I might add other types and containers.

# Examples

Reproducibility yay.
```rust
use arbitrary_rust::Arbitrary;

let obj = (true, false, 1.0);
assert_eq!(
    obj,
    <(bool, bool, f64)>::from_bytes(&obj.to_bytes()[..])
);
```

We can actually write a corpus!
```rust
use arbitrary_rust::Arbitrary;

#[derive(Arbitrary)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn main() {
    let mut buffer = std::fs::File::create("corpus.txt")?;
    buffer.write_all(
        Rgb{
            r:0,
            g:255,
            b:128
        }.to_bytes()
    )?;
}

```

Debugging a crash!
```rust
use arbitrary_rust::Arbitrary;

#[derive(Arbitrary)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn main() {
    let filename = "my_awesome_0day.coprus";
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    let input = Rgb::from_bytes(&buffer[..]);
    println!("LOOK A READABLE CRASH INPUT:\n{:?}", input);
}

```