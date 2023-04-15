# 1. Scalar Types

## 1.1. Integers

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

* When compiling in *debug mode*, Rust includes checks for integer workflow 
    that causes the program to *panic* at runtime if this behaviour occurs.
* But, when compiling in *release mode*, Rust won't include such checks and
    values will *wrap around* in case of overflow. Example, for u8 the value
    256 becomes 0, 257 becomes 1, etc.

## 1.2. Floating-point Numbers

Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits 
in size, respectively.

Integer division rounds down to the nearest integer.

## 1.3. Booleans

*true* and *false*.

## 1.4. Characters

Four bytes in size and represents Unicode Scalar Value.

```Rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

# 2. Compound Types

## 2.1 Tuple

Tuples have a fixed length.
Tuples can contain multiple data types at once.

```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
}
```

## 2.2 Array

Unlike a tuple, every element of an array must have the same type.
Arrays have fixed length.
Data allocated on the stack rather than the heapd (to be learned later).

```
fn main() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    
    let first = a[0];
}
```


