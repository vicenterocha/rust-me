fn main() {
    let x = 5;
    println("{:p}", &x);

    // shadowing
    let x = x + 1;
    println!("{:p}", &x); // different address, therefore different variable

    {
        // temporary shadowing - when the scope is over, x returns to 6
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        println!("{:p}", &x); // different address, therefore different variable
    }

    println!("The value of x is: {x}");
    // Bookmark https://rust-book.cs.brown.edu/ch03-02-data-types.html#floating-point-types
}
