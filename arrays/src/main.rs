use std::io;

fn main() {
    let a = [1];

    let mut index = String::new();

    println!("Please enter an array size");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read a line!");

    let size: usize = index
        .trim()
        .parse()
        .expect("index was not a number");

    let a  = [1; size]; // can't assign a non const to an array, hmnmmm...
    println!(
        "the new array is {}",
        a
    )
}
