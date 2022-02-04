> The final bit of relevant syntax in our Hello, Rust example is that little semicolon. Unlike JavaScript, semicolons in Rust are not optional. Every statement in Rust must end with a semicolon.

semicolons are not optional but I managed to compile without it 🤔

---

`let` variable are immutable by default. use `mut` keyword to make it mutable:

```rust
let mut name = "hello";
```
---

`const` creates a compile time constant value

---

you cannot just simply print an object with `println(obj)`. this will throw this error:
```
error: format argument must be a string literal
  --> main.rs:10:14
   |
10 |     println!(are_you_cool);
   |              ^^^^^^^^^^^^
   |
help: you might be missing a string literal to format with
   |
10 |     println!("{}", are_you_cool);
   |              +++++

error: aborting due to previous error
```
you need to give a "format" as first argument: `println("{}", obj)`
other example :
```rust
println!("Total: {} + {} = {}", price, tax, total);
```
---

- **Signed** and **unsigned** refer to whether it’s possible for the number to be negative
- you cannot add an integer to a float (execept if you can the integer)
---

- In Rust, all elements in an Array must be of the same type.
- you can mix types in a Tuple and mutate its values (with `mut`keyword), but you cannot change its types order once declared.
