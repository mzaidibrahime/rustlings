// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {

    //array of size 100, initialises all the elements to value 20
    let a:[u32;100] = [20; 100];
    // array of size 2, every element is initialised separately
    let b = [1,2];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
    println!("{}, {} size:{}", a[0], a[1], a.len());
    println!("{}, {} size:{}", b[0], b[1], b.len());
}
