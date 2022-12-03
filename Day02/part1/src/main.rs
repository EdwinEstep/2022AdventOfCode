// `cargo run -- search_text ../input.txt`

// args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line
use std::str::SplitWhitespace;


fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    // open file
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);  

    let mut score: u64 = 0;


    // find the largest value, igorning empty strings
    for line in reader.lines() {
        let lstr = line.unwrap().clone();
        let mut row = lstr.split_whitespace();

        let mut round_score = get_score(row.next().unwrap(), row.next().unwrap());
        
        score = score + round_score;
        println!("score:  {}", round_score);
    }


    // print result
    println!("score:  {}", score);
    Ok(())
}


// too explicit with the states
fn get_score(col1: &str, col2: &str) -> u64 {
    // find points depending on whether you won
    (match col1 {
        "A" => match col2 {
                "X" => 3,
                "Y" => 6,
                "Z" => 0,
                _ => 0,
            },
        "B" => match col2 {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            },
        "C" => match col2 {
                "X" => 6,
                "Y" => 0,
                "Z" => 3,
                _ => 0,
            },
        _   => 0,

    // add points for whether you chose rock, paper, or scissors
    }) + (match col2 {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    })
}
