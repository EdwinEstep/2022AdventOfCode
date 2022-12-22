// `cargo run -- ../input.txt`

// args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line


fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    // open file
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);  
    
    let mut assignments = [[0i64; 2]; 2];
    let mut lengths = [0i64; 2];
    let mut i;
    let mut j;

    let mut score: i64 = 0;


    // read line by line
    for line in reader.lines() {
        // make a clone of the line, since the for-loop has ownership
        let lstr = line.unwrap().clone(); 


        // first, split by comma
        // then, split by '-'
        i = 0;
        for elf in lstr.split(',') {
            j = 0;
            for a_num in elf.split('-') {
                assignments[i][j] = a_num.parse().unwrap();
                j += 1;
            }

            lengths[i] = (assignments[i][0] - assignments[i][1]).abs();

            i += 1;
        }
        // println!("{assignments:?}");



        /*
        * 1. take either assignment
        * 2. if it starts before the other assignment, check whether it ends after the start of the other assignment
        * 3. otherwise, check whether it starts before the end of the other assignment
        */
        if assignments[0][0] < assignments[1][0] { // starts before other one starts
            if assignments[0][1] >= assignments[1][0] { // ends after other one starts
                score += 1;
            }
        }
        else if assignments[0][0] <= assignments[1][1] { // starts before end of other assignment
            score += 1;
        }
    }

    println!("score:  {score}");
    Ok(())
}
