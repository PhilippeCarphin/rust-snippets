use std::env;
fn main() {

    // https://doc.rust-lang.org/rust-by-example/std_misc/arg.html
    // env::args() returns an iterator that yields strings
    for a in env::args(){
        println!("argument : {}", a);
    }

    // collect can be used to run through the iterator
    // and get all the strings
    let argv: Vec<String> = env::args().collect();
    for i in 0..argv.len() {
        println!("argv[{}] = {}", i, argv[i]);
    }


}
