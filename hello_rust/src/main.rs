fn main() {
    // Exercises: Create a new Rust project and write a program that:
    // 1. Declares a variable name with your name (type &str)
    // 2. Declares a mutable variable count starting at 0
    // 3. Uses a for loop from 1..=5 to increment count and print "Hello, {name}! (count: {count})"
    // 4. After the loop, print whether count is even or odd using a match expression

    let name: &str = "Gary";

    let mut count: i32 = 0; // let is immutable by default, so it has to be changed to mut

    for _ in 0..=5 {
        // '_' is the place holder, telling the compiler to ignore a value,
        // in this case telling the compiler to evaluate the expression(value increasing in the for loop, but discarding the result afterwards)
        // 1..=x is inclusive range (0-x) similar to pythons range(1, x+1)
        count += 1;

        println!("Hello, {name}!, (count: {count})"); // print statement is very similar to python in terms of formatting: there is no need to have another "" in between each variable
        // even the method of formatting(inserting values) is similar to python, with variables wrapped in {var}
        // unlike python, there is no need to specify that formatting is being applied
    }

    let parity: &str = match count % 2 {
        // the result can also be stored in a variable, in this case: "parity"
        // when the match function is being set up, it can be used to check cases: equal, less than, greater than, etc
        // then the resultant value (output) is whatever the match case is
        0 => "even",
        _ => "odd", // '_' is used to match anything else, meaning that any other value that isn't odd(0) is odd
    };

    println!("Final count: {count} is {parity}")
}
