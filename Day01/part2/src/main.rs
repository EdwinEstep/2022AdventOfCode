// `cargo run -- search_text ../input.txt`

// args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line


// number of elves on snax leaderboard
const NUM_ELVES: usize = 3;


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
    let mut max_snacks: [u64; NUM_ELVES] = [0; NUM_ELVES]; // smaller index -> larger values
    let mut current_snex: u64 = 0;

    // find the largest value, igorning empty strings
    for line in reader.lines() {
        let lstr = line.unwrap().clone();

        // after each elf is processed, see if it's bigger
        if lstr.is_empty() {
            // compare against every value in snack castle (starting with largest)
            for i in 0..NUM_ELVES {
                if current_snex > max_snacks[i] {
                    // push all values down one starting with the lowest element.
                    for n in ((i+1)..NUM_ELVES).rev() {         
                        max_snacks[n] = max_snacks[n-1];
                    }

                    // and insert the snack
                    max_snacks[i] = current_snex;
                    break;
                }
            }

            current_snex = 0;
        }
        else {
            // tabulate snex
            current_snex = current_snex + lstr.parse::<u64>().unwrap();
        }
    }


    // handle final group of snacks
    for i in 0..NUM_ELVES {
        if current_snex > max_snacks[i] {
            // push all values down one starting with the lowest element.
            for n in ((i+1)..NUM_ELVES).rev() {         
                max_snacks[n] = max_snacks[n-1];
            }

            // and insert the snack
            max_snacks[i] = current_snex;
            break;
        }
    }

    // print result
    let sum: u64 = max_snacks.iter().sum();
    println!("{:?}", max_snacks);
    println!("sum:  {}", sum);

    Ok(())
}
