fn main() {
    println!("Sum of evens up to 20: {}", sum_of_evens(20));
}

fn sum_of_evens(n: u32) -> u32 {
    let k = n / 2;
    k * (k + 1)
}
