fn main() {
    for n in 1..=10{
       println!("{n} is {}", if is_even(n) { "even" } else { "odd" });
    }
}

fn is_even(n: i32) ->bool{
    n%2==0
}
