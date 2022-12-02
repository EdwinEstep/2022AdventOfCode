// `cargo run -- search_text input.txt`

// args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line

fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    // open file and set up buff reader
    // 8KB buffer size by default.
    // Or init with `with_capacity(num_bytes, file)` instead of `new()`
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);  

    // useful data
    let mut max_snacks: i64 = 0;
    let mut current_snex: i64 = 0;

    // find the largest value, igorning empty strings
    for line in reader.lines() {
        let lstr = line.unwrap().clone();

        // after each elf is processed, see if it's bigger
        if lstr.is_empty() {
            if current_snex > max_snacks {
                max_snacks = current_snex;
            }

            current_snex = 0;
        }
        else {
            // tabulate snex
            current_snex = current_snex + lstr.parse::<i64>().unwrap();
        }
    }

    // check the final snackbag as well
    // this isn't clean, but it works
    if current_snex > max_snacks {
        max_snacks = current_snex;
    }

    // print result
    println!("{}", max_snacks);

    Ok(())
}
