// Example 1: String ownership
fn example1() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Clone s1 so both variables own their own data

    println!("{}, world!", s1); // Now s1 is still valid
}

// Example 2: Function ownership
fn example2() {
    let s = String::from("hello");
    takes_ownership(&s); // Pass by reference, so s is not moved

    println!("After function call: {}", s); // s is still valid
}

fn takes_ownership(some_string: &String) { // Accept a reference
    println!("Inside function: {}", some_string);
}

// Example 3: Vector ownership
fn example3() {
    let v = vec![1, 2, 3, 4, 5];

    for i in &v { // Iterate by reference, so v is not moved
        println!("{}", i);
    }

    // Calculate and print the sum of elements in v
    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);
}

fn main() {
    println!("Running Example 1:");
    example1();
    
    println!("\nRunning Example 2:");
    example2();
    
    println!("\nRunning Example 3:");
    example3();
}