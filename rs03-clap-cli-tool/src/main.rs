// use clap::{Arg,Command};

use clap::CommandFactory;
use clap::Parser;
use clap_complete;
/*
 * This demonstrates the use of the 'clap' package to handle command line
 * arguments.
 */

/// Description of what my application does
///
/// Can be on multiple lines
#[derive(Parser,Debug)]
pub struct AppOptions {
    /// The file argument for my application
    #[arg(short, long)]
    file: Option<std::path::PathBuf>,
    /// A flag controling a thing in my application
    #[arg(short, long)]
    boolean: Option<bool>,
    /// Generate a SHELL completion script and print to stdout
    ///
    /// which doesn't work yet
    #[arg(short, long)]
    pub generator: Option<clap_complete::Shell>,

}

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}


fn main() {
    let app_options = AppOptions::parse();
    // println!("{:#?}", app_options);
    let matches = AppOptions::command().get_matches();

    match app_options.generator {
        Some(_) => {
                // println!("shell: {:?}", shell);
                if let Some(generator) = matches.get_one::<clap_complete::Shell>("generator") {
                    let mut cmd = AppOptions::command();
                    eprintln!("Generating completion file for {generator}...");
                    print_completions(*generator, &mut cmd);
                }
        },
        None => {
            println!("{:#?}", app_options);
        }
    }
}
