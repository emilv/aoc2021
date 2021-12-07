use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use iterslide::SlideIterator;

fn lines_from_file(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("no file");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("failed line"))
}

fn main() {
    let filename = "in1.txt";
    let contents = lines_from_file(filename);
    let ints = contents
        .into_iter()
        .map(|x| x.parse::<i16>().unwrap())
        .slide(3);
    let mut counter = 0;
    let mut last: Option<i16> = None;
    for w in ints {
        let i = w[0] + w[1] + w[2];
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
