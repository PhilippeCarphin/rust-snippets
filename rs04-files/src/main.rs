use std::fs;
use std::env;
use std::io;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), &'static str> {
    let argv: Vec<String> = env::args().collect();
    // let mut filename = argv.remove(1);
    if argv.len() <= 1 {
        return Err("Need one argument")
    }

    /*
     * Probably what I would want to do most of the time if I were to code
     * a simple program in Rust.
     */
    println!("=============== fs::read_to_string() ===============");
    if let Ok(contents) = fs::read_to_string(argv[1].clone()) {
        println!("{}", contents);
    } else {
        println!("fs::read_to_string({}) returned an error value", argv[1]);
    }
    // And manipulate the string afterwards once the whole file has been read

    /*
     * Simplification of the below thing with their read_lines function into 
     * a single snippet.
     */
    println!("======== File::open() + for (i,l) in BufReader::new(file).lines().enumerage() ... ===============");
    if let Ok(file) = File::open(argv[1].clone()){
        // let Ok(newvar) = X, if X is good, unwrap and put the good value in newvar
        let lines = io::BufReader::new(file).lines();
        // Also, check out this enumerate thing that gives you
        // tuples containing indices and items
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                println!("{}: {}", i+1, ip);
            }
        }
    } else {
        println!("Error opening the file with File::open({})", argv[1]);
    }

    /*
     * "True" line by line reading using an iterator
     *
     * At the time of writing this I don't understand most of the syntax that
     * is going on here and the code of the read_lines() function.
     */
    // From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    println!("======== read_lines() from doc.rust-lang.org  ===============");
    if let Ok(lines) = read_lines(argv[1].clone()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            } else {
                println!("Line not OK");
            }
        }
    } else {
        println!("read_lines({}) (from doc.rust-lang.org) returned an error thing", argv[1]);
    }
    Ok(())

}

// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

