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
        println!("{assignments:?}, lengths={lengths:?}");


        // first, find which is larger (or as large)
        if(lengths[0] >= lengths[1]) {
            i = 0;
        }
        else {
            i = 1;
        }

        // now just need to see if the larger one starts at most small_end - large_length before it the smaller one
        if assignments[i][0] <= assignments[(i+1) % 2][0] { // large one starts before or on small one
            if assignments[i][0] >= (assignments[(i+1) % 2][1] - lengths[i]) { // large one ends on or after small one
                score += 1;
            }
        }
    }

    println!("score:  {score}");
    Ok(())
}
