# Ownership

* Safety is the absence of undefined behavior.
* Rust enforces certain rules at compile time to ensure memory safety.
* Ownership is a way of managing memory in Rust.

## Ownership Rules

* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.
* When a value is dropped, Rust returns the memory to the allocator for reuse.

## Ownership and Functions

* Passing a variable to a function will move or transfer ownership to the function.
* Ownership of the variable will then be returned to the caller when the function completes.
* When ownership is transferred, the original owner of the variable will no longer be able to access it.
* Variables live in frames. A frame is a mapping from variables to values within a single scope, such as a function.
* When an expression reads a variable, the variable's value is copied from its slot in the stack frame.

## References and Borrowing

* Borrowing allows a function to borrow a reference to a variable without taking ownership.
* The borrowed reference can be read or written, but cannot be moved or dropped.
* Mutable references to a variable are exclusive; only one mutable reference is allowed at a time.
* Immutable references can be borrowed multiple times simultaneously.

### Boxes

* The Box<T> type for heap allocation. Allows data to be allocated to the heap.

Move a value from the stack to the heap by creating a Box:

```rust
let val: u8 = 5;
let boxed: Box<u8> = Box::new(val);
```

Move a value from a Box back to the stack by dereferencing:

```rust
let boxed: Box<u8> = Box::new(5);
let val: u8 = *boxed;
```

* **Box deallocation principle**:
    * If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

* Owned data can only be accessed through the owner — no aliases


## Fixing Ownership Errors

### Fixing an Unsafe Program: Returning a Reference to the Stack

```rust
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
```

we are trying to return a reference to whose lifetime ends with the function

1. one solution could be to return the string, not a pointer

```rust
fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}
```

2. another could be to return a static string, if it really is static

```rust
fn return_a_string() -> &'static str {
    "Hello world"    
}
```

3. Another one, completely new to me, is to use a reference-counted pointer

```rust
use std::rc::Rc;
fn return_a_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}
```

which makes use of a garbage collection process to manage the lifetime of the
object. It will only copy the pointer to the data, not the data itself.
In runtime the `Rc` will check the when the last `Rc` pointing to data has been
dropped, then deallocates the memory.
