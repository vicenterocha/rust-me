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


HERE https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html#rust-does-not-permit-manual-memory-management

## Slices

* Slices are a way to borrow a portion of a collection, such as an array or a vector.
* Slices do not have ownership and can be passed around as references.

## Summary

* Ownership is a key feature of Rust for managing memory.
* Rust enforces rules at compile time to ensure memory safety.
* Ownership can be transferred through function calls.
* Borrowing allows a function to borrow a reference to a variable without taking ownership.
* Slices are a way to borrow a portion of a collection without taking ownership.
