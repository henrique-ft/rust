// With use, we can specify a module of a crate that we will use
// The std library is a crate that comes by default with rust and give core functionalities
use std::io;

// This is how declare functions in Rust
// the main function is a special functions
// the mai function is ever the first funcrion called in a Rust system (the entry point to our program)
// Rust use snake case for convention style
fn main() {

    // The println! is a macro (not a function), every time we see a "!" in the final of function, that means that
    // this is a macro
    println!("Hello, world!");
    // Macros dont need to have a strict parameter type, we can send anything to macro parameter.
    // We can see how this function is in internals by using "cargo-expand" lib:
    //  {
         //::std::io::_print(::core::fmt::Arguments::new_v1(
             //&["Hello, world!\n"],
             //&match () {
                 //() => [],
             //},
         //));
     //};


    // Interpolation
    println!("My {} is {}", "age", 24);
    // Do this in a normal function call is impossible

    // You dont need obrigatory set the type, the rust compile is smart enought to infer the type of a
    // variable
    let mars_weight = calculate_weight_on_mars(100.0);
    // All variables in rust are immutable by default if we need a variable that is mutable we have
    // to assing with mut in front:
    // let mut mars_weight

    // OWNERSHIP
    // String is a native struct in Rust
    // And is a complex type that lives on de Heap, and everything that lives on the heap will be
    // cleaned when it goes out of scope
    // Simple types lives in the stack and wont be cleaned
    // Types that lives in the headp must follow this rules in rust:
    //
    // 1. Each value in Rust is owned by a variable
    // 2. When the owner goes out of scope, the value will be deallocated
    // 3. There can only by ONE owner at a time
    let mut input = String::new();

    // The read_line method of io::stdin must receives a reference of a mutable string in params
    io::stdin().read_line(&mut input);

    // When we add & in front a var name meas that we will pass the reference of it to another
    // References are imutable by default, so we have to add &mut to the declaration if we want to
    // have a mutable reference
    let mut oi = String::new();
    ex_mut_ref(&mut oi);
    ex_imut_ref(&oi);
    can_pass_oi(oi);
    //cannot_pass_oi_anymore(oi);
    // REFERENCES RULES:
    // 1. We can have any number immutable references OR one mutable reference and no one immutable
    //    references
    //
    // We cannot do this:
    // let mut x = &mut oi
    // let mut y = &mut oi
    //
    // or
    // let mut x = &mut oi
    // let y = &oi
    //
    // But we can do this:
    // let x = &oi
    // let y = &oi

    println!("Weight on mars: {}kg", mars_weight)
}

fn ex_mut_ref(oi: &mut String) -> () {}
fn ex_imut_ref(oi: &String) -> () {}
fn can_pass_oi(oi: String) -> () {}
//fn cannot_pass_oi_anymore(oi: String) -> () {}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// In Rust, the function signature MUST specify the type of each parameter
// To define the return type, we must set in -> on signature
fn calculate_weight_on_mars2(weight: f32) -> f32 {
    50.0 // is simiar to: return 50.0;
    // In Rust the last value is returned automatically if you set without return and ;
    // It's like Ruby, you can use "return" to return early in functions but in most time we will only return the last value of function
}
