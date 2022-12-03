// `cargo run -- search_text ../input.txt`

// args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line


const ELVES_PER_GROUP: usize = 3;


fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    // open file
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);  

    let mut elf_num = 0;         // track which elf in each group
    let mut chars_used: [u64; ELVES_PER_GROUP] = [0; ELVES_PER_GROUP]; // bits correspond to priorities that have been used.
    let mut shared_chars: u64 = 0;

    let mut score: u64 = 0;


    // read line by line
    for line in reader.lines() {
        // make a clone of the line, since the for-loop has ownership
        // then, get an iterable of the characters in the string
        let lstr = line.unwrap().clone(); 
        let char_iter = lstr.chars();

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

            // set bit corresponding to character received
            chars_used[elf_num] = chars_used[elf_num] | (1 << priority[0]);
        }

        // track which group & which elfwe're handling
        elf_num += 1;
        if elf_num >= ELVES_PER_GROUP {
            // we're at the end of the group, so compare the bitmaps

            shared_chars = chars_used[0];
            for i in 1..ELVES_PER_GROUP {
                shared_chars &= chars_used[i];
            }

            // the cool thing here is that the priority is EXACTLY equal
            // to the number of trailing zeros.
            score += u64::from(shared_chars.trailing_zeros());

            // reset data and move to next group
            elf_num = 0;
            chars_used = [0; ELVES_PER_GROUP];
        }
    }

    // print result
    println!("score:  {}", score);
    Ok(())
}


/*
IDEA:  Use stuff from counting sort

We do a similar thing to part 1, except we need to collect bitmaps for N elves at a time.
Then, just bitwise-and them together, and we're left with the one value shared among all of them.

The priority of this value is just the number of trailing zeros.
*/
