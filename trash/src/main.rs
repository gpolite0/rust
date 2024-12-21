
fn main() {

    // chars
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';  // can print emojis!!! uses ASCII

    println!("{c}, {z}, {heart_eyed_cat}");


    // tuples
    let tuple = (1, 2, 3.4);
    println!("Tuple element 0: {}", tuple.0);
    println!("Tuple element 1: {}", tuple.1);
    println!("Tuple element 2: {}", tuple.2);
}
