// constants don't have fixed addresses
// static variables however do
// these variables can be declared and should be globally

const MEANING_OF_LIFE: u8 = 42; // no fixed address and must specify a type

static Z: i32 = 123; // has memory address

static mut X: i32 = 456; // can make a global static variable mut but DANGEROUS

fn main() {
    // print the global const MoL
    println!("What is the meaning of life? {}", MEANING_OF_LIFE);

    // print static variable
    println!("What is Z? {}", Z);

    // to use X we have to mark the code as unsafe

    unsafe {
        println!("X before change {}", X);
        X = 789;
        println!("X after change {}", X);
    }
}
