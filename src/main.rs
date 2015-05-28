fn main() {
    println!("Rust Types");

    // SUMMARY OF TYPES
    // scalar: boolean, integer, float, char, string, pointer
    // collection: tuple, array, slice
    // composition: struct, enum
    // execution: funtion, closure

    // primitive types
    // bool
    let is_true: bool = true;

    // machine independent types
    // u8, u16, u32, u64
    // i8, i16, i32, i64
    // f32, f64
    let a: u8 = 50;
    let b: i64 = 100;
    let c: f32 = 1.1;
    let d: f64 = 2.2;

    // machine dependent types
    // usize, isize
    let e: usize = 200;
    let f: isize = 250;

    // textual types
    // char, str
    let g: char = 'A';
    let h: &str = "hello world";

    // tuple types
    let i: (char, u8, bool) = ('x', 3, true);

    // array and slice types
    let j: [char; 4] = ['a', 'b', 'c', 'd'];
    let k: &[char] = &j[..];

    // struct types

    // enum types

    // pointer types
    // & - references, * - raw

    // fn types
    fn foo(){}

    // closure types

    // self types
}
