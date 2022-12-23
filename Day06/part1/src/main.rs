// `cargo run -- search_text ../input.txt`

// args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line
use std::collections::VecDeque;

const N: usize = 4;


fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    // open file
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);  

    let mut buf_deq: VecDeque<char> = VecDeque::new(); // store buf of chars
    let mut score = 0;


    // read line by line
    for line in reader.lines() {
        // make a clone of the line, since the for-loop has ownership
        // then, get an iterable of the characters in the string
        let lstr = line.unwrap().clone(); 
        let mut char_iter = lstr.chars();


        // prefill the buffer
        for _i in 0..(N-1) {
            let c = char_iter.next().unwrap();
            buf_deq.push_back(c);
        }


        // check for the key
        let mut index = N;
        let mut dup;
        for c in char_iter {
            buf_deq.push_back(c);

            // dup > 0 indicates that there are duplicate characters
            dup = 0; 
            for i in 0..N {
                for j in 0..N {
                    if i != j && buf_deq[i] == buf_deq[j] {
                        dup += 1;
                    }
                }
            }

            if dup == 0 {
                score = index;
                break;
            }

            // remove the oldest char
            buf_deq.pop_front();
            index += 1;
        }
    }

    println!("score:  {}", score);
    Ok(())
}
