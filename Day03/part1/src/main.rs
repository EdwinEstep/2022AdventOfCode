// `cargo run -- search_text ../input.txt`

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

    let mut score: u64 = 0;


    // read line by line
    for line in reader.lines() {
        // make a clone of the line, since the for-loop has ownership
        // then, get an iterable of the characters in the string
        let lstr = line.unwrap().clone(); 
        let char_iter = lstr.chars();

        let compartment_size = lstr.chars().count() >> 1;
        let mut index = 0;
        let mut chars_used: u64 = 0; // bits correspond to priorities that have been used.

        for c in char_iter {
            let mut priority = [0; 1];
            
            // essentially extract ASCII rep and shift down
            if c.is_ascii_uppercase() {
                c.encode_utf8(&mut priority);
                priority[0] = priority[0] - 38;
            }
            else if c.is_ascii_lowercase() {
                c.encode_utf8(&mut priority);
                priority[0] = priority[0] - 96;
            }

            // For the first half of the bag, just set a bit in the chars_used bit array
            if index < compartment_size {
                chars_used = chars_used | (1 << priority[0]);
            }
            else if (chars_used & (1 << priority[0]))  != 0 {  // In the second act, we check each priority to see if it has been used
                // update score and short-circuit the loop
                score += u64::from(priority[0]);
                break;
            }

            index += 1;
        }
    }

    println!("score:  {}", score);
    Ok(())
}


/*
IDEA:  Use stuff from counting sort

There are 52 states, and we just need to know which ones are used in the right side vs
the left side of the bag. So, the procedure goes like this:

1. Read a line
2. Separate into characters
3. While reading the left half, set a bit for every priority level used.
4. Then, when reading the left half, check whether the priority has been used in the right half.
5. If the priority has been used before, tally it up and short-circuit the loop.

*/
