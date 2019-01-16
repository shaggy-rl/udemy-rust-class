use std::mem;

fn main() {
    // unsigned 0..255 and inmutable
    let a:u8 = 128; // 8 bits
    println!("a = {}", a);

    // mutable signed 8bits
    let mut b:i8 = 0;
    println!("b is {}", b);
    b = 42;
    println!("b is now {}", b);

    // can specify variables without declaring type
    let mut c = 123456789; // 32-bit signed i32
    println!("c is {}, size is {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c is now {}", c);
    // i8 u8 i16 u16 i32 u32 i64 u64 are data types usable in Rust

    let z:isize = 156; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z is {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d is {}, with size {} bytes", d, mem::size_of_val(&d));

    let e:f64 = 2.5; // double-precision, 8 bytes or 64 bits, f64
    println!("e is {}, with size {} bytes", e, mem::size_of_val(&e));

    // true and false
    let g = false;
    println!("g is {}, with size {} bytes", g, mem::size_of_val(&g));
}
