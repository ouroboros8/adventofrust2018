use std::env;
use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Please supply a filename");
    println!("Reading input data from {}", filename);

    let mut inputfile = File::open(filename).expect("File not found");
    let mut contents = String::new();
    inputfile
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let input = contents
        .trim()
        .split('\n')
        .map(|x| {
            x.parse::<isize>()
                .expect("Failed to convert input to numbers (isize)")
        });

    let result: isize = input.clone().sum();
    println!("The sum is {}", result);

    let partial_sums = input.cycle().coalesce({|prev, cur|
        Err((prev, prev + cur))
    });

    let mut seen: Vec<isize> = Vec::new();
    let first_repeat = partial_sums.clone().find(|&x| {
        let result = seen.contains(&x);
        seen.push(x);
        result
    }).unwrap();
    println!("First repetition is {}", first_repeat)
}
