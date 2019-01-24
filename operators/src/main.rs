use std::mem;

fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1;
    println!("{}", a);
    a -= 2; // -= += *= /= %=
    println!("{}", a);
    println!("remainder of {} / {} = {}", a, 3, (a % 3));
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                   // 01 or 10 = 11 == 3_10
    println!("1 | 2 = {}", c);
    let two_to_ten = 1 << 10; // shift operator
    println!("2^10 = {}", two_to_ten);

    // logical
    let pi_less_four = std::f64::consts::PI < 4.0; // true
    // can have > <= >= ==
    println!("Is PI less than four? {}", pi_less_four);
}

fn main() {
    operators()
}

