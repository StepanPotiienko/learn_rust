use std::fmt::Display;

fn main()  {
    // No type annotation, no fixed size. Just pure madness.
    let tuple = ("hello", "2", '1', true, 1);
    
    // Type and size are required. 
    // Changed from i8 to i32 because some numbers were greater than i8 max range.
    let array: [i32; 5] = [1, 23, 133, 4512, 56129312];

    // How different things can be.
    println!("{}", tuple.0);
    println!("{}", array[0]);

    // Slicing array like a cake.
    println!("{:?}", &array[1..4]);
    print_array(&array);
}

// We use <T: Display> to ensure that we accept arrays with any type, not just one particular.
fn print_array<T: Display>(array: &[T]) {
    for i in 0..array.len() {
        match array.get(i) {
            Some(val) => println!("{} : {}", i, val),
            None => println!("Nothing lol."),
        }
    }
}

