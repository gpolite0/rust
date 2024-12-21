
fn main() {

    // chars
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';  // can print emojis!!! uses ASCII

    println!("{c}, {z}, {heart_eyed_cat}");


    // tuples - can be diff types
    let tuple = (1, 2, 3.4);
    println!("Tuple element 0: {}", tuple.0);
    println!("Tuple element 1: {}", tuple.1);
    println!("Tuple element 2: {}", tuple.2);


    // arrays - fixed size, all same type, allocated on stack
    let array = ["Jan", "Feb", "Mar"];
    let array_size = array.len();

    for i in 0..array_size {
        println!("Array at index {i}: {}", array[i]);
    }


    // vectors - variable size, all same type, allocated in memory

    // one way to define a vector
    let mut vector1 = vec![1, 2, 3];
    vector1.push(4);

    // another way to define a vector
    let mut vector2 = Vec::new();
    vector2.push(1);
    vector2.push(2);
    vector2.push(3);
    vector2.push(4);

    // indexing a vector
    println!("The second elem is {}", vector1[1]);

    // print all values
    println!("\nog values");
    for val in &vector2 {
        println!("{val}");
    }

    // multiply all values by two
    for i in &mut vector2 {
        *i *= 2;
    }

    println!("\nnew values");
    // print again w new values
    for val in &vector2 {
        println!("{val}");
    }
}
