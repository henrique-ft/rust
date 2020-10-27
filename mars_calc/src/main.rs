
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

    calculate_weight_on_mars(100.0);
}

// In Rust, the function signature MUST specify the type of each parameter
// To define the return type, we must set in -> on signature
fn calculate_weight_on_mars(weight: f32) -> f32 {
    50.0 // is simiar to: return 50.0;
    // In Rust the last value is returned automatically if you set without return and ;
    // It's like Ruby, you can use "return" to return early in functions but in most time we will only return the last value of function
}
