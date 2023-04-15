# 1. Functions

Interesting usage of scope block. It returns a value. Its value is assigned to 
the variable y.

```Rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```
