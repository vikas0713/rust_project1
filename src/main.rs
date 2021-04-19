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
    // Two types of variables ; signed and unsigned
    // length could be: 8,16,32,64, 128 or arch


}
