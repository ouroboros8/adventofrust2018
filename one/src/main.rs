use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Please supply a filename");
    println!("Reading input data from {}", filename);

    let mut inputfile = File::open(filename).expect("File not found");

    let mut contents = String::new();
    inputfile
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let input: Vec<i32> = contents.trim().split('\n').map(|x| {
        x.parse::<i32>()
            .expect("Failed to convert input to numbers (i32)")
    }).collect();

    let result: i32 = input.iter().sum();
    println!("The sum is {}", result);
}
