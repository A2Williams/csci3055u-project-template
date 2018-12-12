fn main() {
    let greeting = "Hello World";
    println!("{}", greeting);

    println!("Say hello to {0}, {1}", "the world", "Aaron!");

    println!("{name} says hello to the {place}.",
             name = "Aaron",
             place = "world");
}