# Prelude

https://doc.rust-lang.org/std/prelude/index.html

The prelude is a set of resources that Rust automatically imports on every program.

# Associated Function

An associated function is a function that’s implemented on a type.
Example `String::new();`.

# Result

https://doc.rust-lang.org/std/result/enum.Result.html

Enum std::result::Result

```Rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Result is a type that represents either success (Ok) or failure (Err).
