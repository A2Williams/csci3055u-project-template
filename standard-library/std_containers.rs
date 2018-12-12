use std::collections::HashMap;

fn main() {
    // vectors exists in the std
    let mut vector = vec![11,22,33,44];
    println!("Initial vector: {:?}", vector);
    
    // vector containers come with a length value and indexing
    println!("Size of vector is {}. the first element is {}", vector.len(),vector[0]);
    
    // vectors can grow and shrink in size
    vector.push(55);
    println!("vector after push: {:?}", vector);
    println!("vector popped last element: {:?}",vector.pop());

    // vectors can be iterated over and enumerated
    for (i,x) in vector.iter().enumerate() {
        println! ("position {} = {}", i, x);
    }

    // String data are handled in UTF-8
    let hello = "こんにちは";

    // Strings are not simple to index as unicodes have scalar values
    println!("The first character in the hello variable is: {}", &hello[0..3]);

    // Hash Maps also exists
    let mut kanji_ref = HashMap::new();
    
    // Hash Maps are also growable
    kanji_ref.insert("Ko",&hello[0..3]);
    kanji_ref.insert("N", &hello[3..6]);
    kanji_ref.insert("Ni", &hello[6..9]);
    kanji_ref.insert("Chi", &hello[9..12]);
    kanji_ref.insert("Wa", &hello[12..15]);

    // Hash Maps are also iterable
    for (key,val) in &kanji_ref {
        println!("{} in kanji is: {}", key, val);
    }

}