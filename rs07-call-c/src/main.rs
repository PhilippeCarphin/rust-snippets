use cty;

extern "C" {
    pub fn print_string_list(
        argc: cty::c_int,
        argv: *const *const cty::c_uchar
    );
}

fn main() {
    println!("Hello, world!");
    unsafe {
        print_string_list(2,["allo\0".as_ptr(), "bonjour\0".as_ptr()].as_ptr());

        print_string_list(2,["It's the eye of the tiger".as_ptr(), "it's the thrill of the fight".as_ptr()].as_ptr());
    }
}
