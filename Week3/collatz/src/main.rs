
fn collatz(n:i32)->i32{
    print!("The sequence is: ");
    let mut n = n;
    let mut counter=0;
    loop {
        if n==1 {
            println!(" 1.");
            break counter
        }
        print!(" {}", n);
        n=if n%2==0 {n/2} else {n*3+1};
        counter+=1;
    }
}

fn main() {
    println!("We need {} steps for number {}.", collatz(6), 6);
}