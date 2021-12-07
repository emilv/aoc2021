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
    let filename = "input";
    let contents = lines_from_file(filename);
    let directions = contents
        .map(|x| {
            let (a, b) = x.split_once(' ').unwrap();
            (
                a.to_string(),
                b.parse::<i32>().unwrap()
            )
        });

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for (dir, x) in directions {
        match dir.as_ref() {
            "forward" => horizontal += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => panic!("dont now this")
        }
    }
    println!(
        "depth={}, horizontal={}, multiplied={}",
        depth,
        horizontal,
        depth * horizontal
    );
}

