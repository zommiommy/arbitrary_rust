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

The next step will be introducing automatic derivation with `#[derive(Arbitrary)]`, once I figure out were people hide the documentation about procedural macros.

# Examples

Reproducibility yay.
```rust
use arbitrary_rust::Arbitrary;

assert_eq!(
    (true, false, 1.0),
    <(bool, bool, f64)>::from_bytes(&(true, false, 1.0).to_bytes()[..])
);
```

We can actually write a corpus!
```rust
use arbitrary_rust::Arbitrary;

#[derive(Copy, Clone, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Arbitrary for Rgb {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut self.r.to_bytes()
        );
        result.append(
            &mut self.g.to_bytes()
        );
        result.append(
            &mut self.b.to_bytes()
        );
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]){
        let (r, data) = u8::build_from_bytes(data);
        let (g, data) = u8::build_from_bytes(data);
        let (b, data) = u8::build_from_bytes(data);
        (
            Rgb{r, g, b},
            data
        )
    }
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

#[derive(Copy, Clone, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Arbitrary for Rgb {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(
            &mut self.r.to_bytes()
        );
        result.append(
            &mut self.g.to_bytes()
        );
        result.append(
            &mut self.b.to_bytes()
        );
        result
    }

    fn build_from_bytes(data: &[u8]) -> (Self, &[u8]){
        let (r, data) = u8::build_from_bytes(data);
        let (g, data) = u8::build_from_bytes(data);
        let (b, data) = u8::build_from_bytes(data);
        (
            Rgb{r, g, b},
            data
        )
    }
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