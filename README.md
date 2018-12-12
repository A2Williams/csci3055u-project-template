# A look at the RUST language 
- Aaron Williams
- aaronchristoph.williams@uoit.net

# About the language

### **History**
Rust was a personal project started by Graydon Hoare, a Mozilla employee back in 2006. It's namesake possibly takes from the rust family of fungi. In 2009 Mozilla began sponsoring the project and announced it in 2010. It was around this time that work shifted from using the inital compiler written in OCaml to the self-hosting compiler named rustc written entirely in Rust. It successfully compiled itself in 2011.

### **Features**
- Rust gurantees reliability with memory-safety and thread-safety via. its rich type system and ownership model.

- Rust has an included dependency manager and build tool named ```cargo```. 

- Rust can ensure coding style consistency with ```rustfmt```.

- The Rust compiler's checks ensure stability through feature additions and refactoring.

# About the syntax
Rust shares a similar syntax to C and C++:
```rust
fn main() {
    let greeting = "Hello World";
    println!("{}", greeting);
}
```
variables are created through the ```let``` statement. This creates an immutable variable by default. Mutable variables can be created by using ```let mut``` instead. 

**Interesting Fact:** The compiler will throw a warning if your variable is mutable and does not need to be.

Rust also allows for variable shadowing so it is easier to convert variables while using the same variable name.

``if`` statement conditions don't require parentheses! The same applies to ``for`` loops.

 The ``for`` loop works similar to python and allows for a range that is inclusive on both sides:
```rust
// print all numbers from [0..10]
for i in 0..=10 {
    print!("{} ", n);
}
```

# About the tools

_Describe the compiler or interpreter needed_.

The Rust compiler is invoked in command line as ```rustc```. Compilation of code is also similar to C and C++:
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
```rustc``` comes packaged with the Rust language.

# About the standard library

_Give some examples of the functions and data structures
offered by the standard library_.

The Rust standard library (``std::*``) is home to the collections module which defines maps, sets, linked lists, hash maps, and other typical collection types seen in common languages.

Rust ``std`` also includes common types of I/O modules that include files, TCP and UDP. It also contains the thread module which deals with multithreading operations.

# About open source library

One interesting open source library made by the community is ``astro-rust``. 

``astro-rust`` is a rust library created by **saurvs** which contains a library full of advanced astronomical algorithms for Rust. This includes:

- planetary and solar positioning by the complete set of element of Bretagnon and Francou's VSP087 theory
- satellite positioning for Saturn and Jupiter
- corrections for precession, nutation, parallax, aberration, atmospheric refration
- and much more

the library can be found [here](https://github.com/saurvs/astro-rust).

# Analysis of the language

> _Organize your report according to the project description
document_.

Like most languages, Rust has reserved keywords used for certain functions. However for compatibility reasons, it is possible to use such keywords using ```raw identifiers```.
