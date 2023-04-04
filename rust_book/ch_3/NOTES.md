# Scalar Types

## 1. Integers

An integer is a number without a fractional component

```
Length  Signed  Unsigned
8-bit   i8      u8
16-bit  i16     u16
32-bit  i32     u32
64-bit  i64     u64
128-bit i128    u128
arch    isize   usize
```

The primary situation in which one could use *isize* or *usize* is when indexing some sort of collection.

A note on **integer overflow**:

* When compiling in *debug mode*, Rust includes checks for integer workflow that causes the program to *panic* at runtime if this behaviour occurs.
* But, when compiling in *release mode*, Rust won't include such checks and values will *wrap around* in case of overflow. Example, for u8 the value 256 becomes 0, 257 becomes 1, etc.

## 2. Floating-point Numbers

https://rust-book.cs.brown.edu/ch03-02-data-types.html#floating-point-types

## 3. Booleans

## 4. Characters

# Compound Types

