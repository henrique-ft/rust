// Stack: is the local / scope memory that lives inside a function and its cleaned automatically
//
// Heap: is the global memory that is accessible by any point of the code, but is managed "by hand"
// and must be cleaned or we can have the "memory heap error". In rust we have a easy abstraction
// for this called "Box"

fn main() {
    let a = 2;
    let result = stack_only(a);
    dbg!(result);
    basic_data_types();
}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    let d = 5;
    let e = Box::new(7);

    return d + dbg!(*e);
}

fn basic_data_types() -> () {
    // INTEGERS
    println!("INTEGERS");
    let a: u8 = 1; // this is a unsigned 8 bit integer
    let b: i8 = 1; // this is a signed 8 bit integer
    println!("{}", a);
    println!("{}", b);

    // unsigned integer means: it only can be positive
    // unsigned integer means: it can be positive and negative
    // the number after u or i are the number of bits, and in rust we have 8, 16, 32, 64 and 128

    // special types of numbers: usize and isize, they are architecture independent "size" will be
    // 32 bits in a 32 bits architecture or 64 bits in a 64 bits architecture

    // FLOATS
    println!("FLOATS");
    let im_a_float_32_bits: f32 = 9.0;
    let im_a_float_64_bits: f64 = 5.0;
    println!("{}", im_a_float_32_bits);
    println!("{}", im_a_float_64_bits);

    // BOOLEAN
    println!("BOOLEAN");
    // It is a 1 bit value
    let true_or_false: bool = true;
    println!("{}", true_or_false);

    // CHAR
    println!("CHAR");
    // holds a single unique value that is always 4 bits
    // char also are valid u32 integer
    let name: char = 'o';
    println!("{}", name);
    dbg!(name as u32);
}
