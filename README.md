Macro to implements the `Debug` trait that prints nothing.

Example usage:
`Rust
use nodebug_derive::nodebug;

struct MyType {
    // fields...
}
nodebug!(MyType);

fn main() {
    println!("{:?}", MyType{/*...*/}); // prints: MyType::debug
}
`
