// scope is defined by {}
// variables only exist within the {}
// if {} inside {}, then the variables in the parent {}
// are available in the child {} scope outlined below
fn scope_and_shadowing() {
    let a = 123;
    println!("a = {}", a);

    // a is avaiable in the scope below and will be the same as parent scope
    {
        let b = 456;
        println!("inside, b = {}", b);
        // a is in parent scope so available in this this scope
        println!("inside, a = {}", a);
    }

    // new scope will be created that will show the veriable 'a'
    // with a new assigned vaule. the variable will be created within this scope
    // and have a different value than the 'a' in the parent scope
    {
        let a = 666;
        println!("inside second scope, a = {}", a);
    }
}

fn main() {
    scope_and_shadowing();
}
