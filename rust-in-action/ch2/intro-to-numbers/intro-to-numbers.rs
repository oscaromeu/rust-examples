fn main() {
    let twenty = 20;            // rust infers a type on your behalf if you don't supply one
    let twenty_one: i32 = 21;   // number with type annotations
    let twenty_two = 22i32;     // number with type suffixes

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000; // underscores increase readability and are ignored by the compiler
    println!("{}", one_million.pow(2)); // numbers have methods
    
    // creating an array of numbers, which all be the same type by surrounding those with
    // square brackets
    let forty_twos = [
        42.0,                
        42f32,    // floating point literals can also have type suffixes
        42.0_f32, // floating point literal with optional underscores

    ];

    // Elements within arrays can be indexed numericaly, starting at 0
    println!("{:02}", forty_twos[0]);


}