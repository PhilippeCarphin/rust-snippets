// build.rs, in the project root folder
fn main() {
    println!("cargo:rustc-link-search=all=C-prog");      // works like "rustc -L src ..." 
    println!("cargo:rustc-link-lib=dylib=printlist"); // works like "rustc -l doubler.o"
}
