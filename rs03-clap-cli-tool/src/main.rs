use clap::{Arg,Command};
/*
 * This demonstrates the use of the 'clap' package to handle command line
 * arguments.
 */
fn main() {
    let my_app_args = Command::new("My App Name")
        .version("0.0.1")
        .author("Philippe Carphin")
        .arg(Arg::new("file")
             .short('f')
             .long("file")
             .takes_value(true)
             )
        .arg(Arg::new("flag")
             .short('g')
             .long("flag")
             .takes_value(false)
             )
        .get_matches();

    let file = my_app_args.value_of("file").unwrap_or("");

    println!("The value of --file (-f) is {}", file);


}


