fn main() {
    // 1. Declare an immutable integer variable
    let my_integer = 42;

    // 2. Declare a mutable float variable and modify it later
    let mut my_float = 3.14;
    
    // Modify the float value
    my_float = my_float + 2.0;

    // 3. Declare a boolean variable using type inference
    let is_rust_fun = true;
    
    // 4. Declare a character variable with explicit type annotation
    let my_char: char = 'R';

    // 5. Perform arithmetic operations with the numeric variables
    let sum = my_integer as f64 + my_float;
    let product = my_integer as f64 * my_float;
    
    // 6. Print all variables and calculation results with appropriate labels
    println!("Integer value: {}", my_integer);
    println!("Modified float value: {}", my_float);
    println!("Boolean value: {}", is_rust_fun);
    println!("Character value: {}", my_char);
    println!("Addition result: {}", sum);
    println!("Multiplication result: {}", product);
}