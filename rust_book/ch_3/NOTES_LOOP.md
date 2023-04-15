# Loops

There are three types of loops: `loop`, `while` and `for`.

## loop

The loop keyword tells Rust to execute a block of code over and 
over again forever or until you explicitly tell it to stop

On the following script we can see a loop where the result is assigned
to the variable `result`.

```Rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // the result of the expression after the break is returned on the loop
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

Working with labels for nested loops.

```Rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // this will break the outloop as well
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```


## while


```Rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

## for

```Rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

Reversing order `.rev()`

```Rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
