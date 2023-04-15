use std::result::Result;
use std::error::Error;

/*
 * Three ways of launching a process:
 * - Using output(): launch and wait with captured output
 * - Using spawn(): Launch asynchronously
 * - Using status(): Launch and wait without capturing output.
 */
fn main() -> Result<(),Box<dyn Error>> {
    /*
     * Run a command and collect its output
     */
    let mut c1 = std::process::Command::new("/bin/bash");
    c1.args(["-c", "echo hello world"]);
    let o = c1.output()?;
    println!("{}", std::str::from_utf8(&o.stdout)?);
    // NOTE: This doesn't work because methods seem to take ownership of self
    // and return self:
    // let mut c1 = std::process::Command::new("/bin/bash").args(["-c", "echo hello world"]);
    // temporary value dropped while borrowed creates a temporary which is freed while still in use [E0716]
    // however, in the next example, since we end the chain with spawn, we can do it.

    /*
     * Start a child process and let it go while we do other stuff
     */
    let f = std::fs::File::create("output_file.txt")?;
    let mut child = std::process::Command::new("/bin/bash")
        .args(["-c", "for((i=0;i<4;i++)); do echo \"$$: i=$i\"; echo \"$$: i=$i\" >&2 ; sleep 1; done"])
        .stdout(f)
        // .stderr(std::process::Stdio::piped())
        .spawn()?;
    println!("main(): Waiting for child {:#?} (PID = {}) to finish", child, child.id());
    child.wait()?;
    println!("main(): Child {:#?} (PID = {}) has finished", child, child.id());

    let handle = std::thread::spawn(|| {
        println!("Thread: Launching process synchronously");
        let f = std::fs::File::create("output_file.txt").expect("Error creating file");
        let exit_status = std::process::Command::new("/bin/bash")
            .args(["-c", "for((i=0;i<4;i++)); do echo \"$$: i=$i\"; echo \"$$: i=$i\" >&2 ; sleep 1; done"])
            .stdout(f)
            // .stderr(std::process::Stdio::piped())
            .status().expect("Error running process");
        println!("Thread: process finished with status {:#?}", exit_status);
    });
    println!("main(): Thread launched: {:#?}: TID:{:#?}", handle, handle.thread().id());
    println!("main(): Waiting for thread to finish");
    handle.join().expect("Error waiting for thread");
    Ok(())
}
