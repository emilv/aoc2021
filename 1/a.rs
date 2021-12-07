use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("no file");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("failed line"))
}

fn main() {
    let filename = "in1.txt";
    let contents = lines_from_file(filename);
    let ints = contents.into_iter().map(|x| x.parse::<i16>().unwrap());
    let mut counter = 0;
    let mut last: Option<i16> = None;
    for i in ints {
        match last {
            None => { },
            Some(j) => {
                if i > j {
                    counter += 1;
                }
            }
        }
        last = Some(i);
    }
    println!("{}", counter);
}
