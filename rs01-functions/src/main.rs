fn my_normal_function(n: i32, s: &str) -> i32 {
    for i in 0..8 {
        println!("i = {}, {}", i, s);
    }
    return n;
}

fn main() {
    let a = my_normal_function(8, "Hello world");
    println!("the value of a is {}", a);
    assert_eq!(a, 8);
}
