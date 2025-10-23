fn main() {
    println!("Max is: {}", max_of_three(3, 7, 5));   
}

fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
   let mut max = a;
    if b > max { max = b; }
    if c > max { max = c; }
    max
}
