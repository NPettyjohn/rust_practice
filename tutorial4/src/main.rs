fn main() {
    
    // integer types
    // i8   u8
    // i16  u16
    // i32  u32
    // i64  u64
    // i128 u128
    let x: i32 = 2;

    // floating point
    // f32 f64

    // boolean
    let true_or_false: bool = false;

    // character
    let character: char = 'a';

    // tuple
    let mut tup: (i32, bool, char) = (1, true, 'a');
    // let tup2: (i8, bool, char) = (1, true, 'a');

    tup.0 = 10;

    println!("tup value is: {}", tup.0);


    // arrays
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 3;
    println!("array value is: {}", arr[4]);



}
