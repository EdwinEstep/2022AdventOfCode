// Goal:  find the  
//
// `cargo run -- ../input.txt`

// args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line
use std::collections::VecDeque;

const NUM_STACKS: usize = 9;


fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    // parse initial data from file
    let file = File::open(fpath)?;
    let mut reader = BufReader::new(file);  
    let mut file_lines = reader.lines().map(|l| l.unwrap()); // get iterator of lines
    
    let mut instr: [usize; 3] = [0, 0, 0];
    let mut i: usize;

    // read in the initial stacks
    let mut stacks =  parse_stacks(&mut file_lines);
    file_lines.next(); // ignore empty line

    // read line by line, operating on each command
    for line in file_lines {
        let lstr = line.clone(); 
        // println!("=> {}", lstr);

        let mut words = lstr.split_whitespace();
        words.next();

        // the numbers occur every other word
        i = 0;
        for w in words.step_by(2) {
            instr[i] = w.parse::<usize>().unwrap();
            i += 1;
        }
        println!("{instr:?}");


        // perform command
        for j in 0..instr[0] {
            let a = stacks[instr[1] - 1].pop_front().unwrap();
            stacks[instr[2] - 1].push_front(a);
        }

        println!("{stacks:?}");
    }

    // assemble score
    let mut score = String::from("");
    for c in 0..NUM_STACKS {
        score.push(*stacks[c].front().unwrap());
    }

    println!("score:  {score}");
    Ok(())
}

fn parse_stacks<T: AsRef<str>>(args: &mut impl Iterator<Item = T>)  -> Vec<VecDeque<char>> {
    // make array of vectors
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for i in 0..NUM_STACKS {
        stacks.push(VecDeque::new());
    }

    // read line by line
    let mut i: usize;
    for line in args {
        let lstr = line.as_ref(); 

        // stop when the crate list has been read in
        if lstr.chars().nth(1).unwrap() == '1' {
            break;
        }

        // get the char for each crate in the stack
        let mut str_chars = lstr.chars();
        str_chars.next();                 // remove first char

        i = 0;
        for c in str_chars.step_by(4) {   // not robust, assumes exact placement
            // print!("{c} ");

            // if it's not a space, it should be put placed on the vector.
            if c != ' ' {
                stacks[i].push_back(c);
            }

            i += 1;
        }
        // println!(" <= ");
    }

    println!("{stacks:?}");

    stacks
}



/*
NOTES:
- Each letter in the input is separated by three extraneous characters.
- Can represent the stacks as an array of linked lists.
- Array length is the line number of the first empty line minus two.
-  

For this problem I had code to infer the size of buffer to use, but I ended up
hard-coding it for convenience.
*/
