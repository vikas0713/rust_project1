fn main() {
    println!("Hello, world!"); // println macro (! signifies it's a macro)
    const MAX_LIMIT: i32 = 100_000;
    let mut variable1 = 356;
    println!("Max Limit is : {}", MAX_LIMIT);
    println!("Mutable variable is {}", variable1);
    variable1 = 234;
    println!("Changed Variable name: {}", variable1);
    // Shadowing
    let var2 = "  ";
    let var2 = var2.len();
    let var2 = var2 + 1;
    println!("Variable 2 value after addition: {}", var2);
    let var2 = var2 * 2;
    println!("Variable 2 after multiplication: {}", var2);
    println!("{}", var2);

    // Data types
    let signed_variable: i32 = -12343;
    let unsigned_variable: i32 = 1234567;
    let floated_number: f32 = 5.00;     // or can use f64
    let bool_var = true;    // or can nbe false\

    // Two types of variables ; signed and unsigned
    // length could be: 8,16,32,64, 128 or arch

    // addition
    let sum = 5+25;

    // subtraction
    let difference = 50-45;

    // Multiplication
    let product = 67*100;

    // Compound types
    //tuple
    let x = (1,2,3,4);
    println!("Tuple: {}", x.0);
    println!("Tuple: {}", x.1);
    println!("Tuple: {}", x.2);

    // Array
    let z = [1,2,3,4,5];
    println!("Array: {}", z[0]);
    println!("Array: {}", z[1]);
    println!("Array: {}", z[2]);

    // Functions
    println!("Function Call: {}", add_hundred(400));

}


fn add_hundred(x: i32)-> i32 {
    let z = x + 100;
    z       // No semi-colon in case of return from a function
}
