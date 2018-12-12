# A look at the RUST language 
- Aaron Williams
- aaronchristoph.williams@uoit.net

# About the language

### **History**
Rust was a personal project started by Graydon Hoare, a Mozilla employee back in 2006. It's namesake possibly takes from the rust family of fungi. In 2009 Mozilla began sponsoring the project and announced it in 2010. It was around this time that work shifted from using the inital compiler written in OCaml to the self-hosting compiler named rustc written entirely in Rust. It successfully compiled itself in 2011.

### **Features**
- Rust gurantees reliability with memory-safety and thread-safety via. its rich type system and ownership model.

- Rust has an included dependency manager and build tool named `cargo`. 

- Rust can ensure coding style consistency with `rustfmt`.

- The Rust compiler's checks ensure stability through feature additions and refactoring.

# About the syntax

- Rust shares a similar syntax to C and C++:
```rust
fn main() {
    let greeting = "Hello World";
    println!("{}", greeting);
}
```
- variables are created through the `let` statement. This creates an immutable variable by default. Mutable variables can be created by using `let mut` instead. 

**Interesting fact:** The compiler will throw a warning if your variable is mutable and does not need to be.

- Rust also allows for variable shadowing so it is easier to convert variables while using the same variable name.

- `if` statement conditions don't require parentheses! The same applies to `for` loops.

 - The `for` loop works similar to python and allows for a range that is inclusive on both sides:
```rust
// print all numbers from [0..10]
for i in 0..=10 {
    print!("{} ", n);
}
```

# About the tools

The Rust compiler is invoked in command line as `rustc`. Compilation of code is also similar to C and C++:
```
rustc hello_world.rs
```
This generates a executable binary file, which can executed using:
```
./hello_world
```
on linux systems or:
```
.\hello_world.exe
```
on windows systems.
`rustc` comes packaged with the Rust language.

# About the standard library

- The Rust standard library (`std::*`) is home to the collections module which defines maps, sets, linked lists, hash maps, and other typical collection types seen in common languages.

- Rust `std` also includes common types of I/O modules that include files, TCP and UDP. It also contains the thread module which deals with multithreading operations.

# About open source library

One interesting open source library made by the community is `astro-rust`. 

`astro-rust` is a rust library created by **saurvs** which contains a library full of advanced astronomical algorithms for Rust. This includes:

- planetary and solar positioning by the complete set of element of Bretagnon and Francou's VSP087 theory
- satellite positioning for Saturn and Jupiter
- corrections for precession, nutation, parallax, aberration, atmospheric refration
- and much more

the library can be found [here](https://github.com/saurvs/astro-rust).

# Analysis of the language

- Rust is primarily a **procedural** programming language.

- Rust has the ability to preform meta-programming through macros (`print!` is an example of a Rust macro).

- Symbol resolution in Rust is done through a featur known as `ownership`. Memory is manged through a system of ownership with a set of rules the compiler checks at runtime

- Ownership rules consist of the following:
>1. Each value has a variable that's called it's *owner*
>2. There can only be one owner at a time
>3. When the owner goes out of scope, the value is dropped

- Rust supports closure which is syntactically similiar to Ruby. closures are defined by a pair of vertical bars (`|`), with the parameters to the closure specified inside:
```rust
let closure = |num| {
    print!("the number is {} in this closure!", num);
};
```
**Another interesting fact:** if a piece of code was to call a closure with one value, the compiler infers the data type of the closure and return type.

- Scoping in rust is `lexical`. The example below would print `"Hi world"`:
```rust
    fn hello() -> String {
        let greeting = String::from("Hi world");
        greeting
    }
    let greeting = "Hello World";
    println!("{}", hello());
```
- Rust does support functional programming constructs as part of language through the use of `closures` and `iterators`.

- Rust allows for both static and dynamic types but are static by default.

#### pros
- Very efficient compiler which is also.user-friendly.
- Lots of tools available on install/
- Syntax draws many similarities to other languages. Primarily C/C++.
#### cons
- Very strict compiler.
- Lots of material to learn in order to become productive.
- Ownership memory management can be very confusing to learn.
