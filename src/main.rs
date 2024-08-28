fn main()  {
    // No type annotation, no fixed size. Just pure madness.
    let tuple = ("hello", "2", '1', true, 1);
    
    // Type and size are required. 
    let array: [i8; 5] = [1, 2, 3, 4, 5];

    // How different things can be.
    println!("{}", tuple.0);
    println!("{}", array[0]);

    // Slicing array like a cake.
    println!("{:?}", &array[1..4]);

    // Every Python developer worst nightmare.
    for i in 0..array.len() {
        match array.get(i) {
            Some(val) => println!("{} : {}", i, val),
            None => println!("Nothing lol."),
        }
    }
}
