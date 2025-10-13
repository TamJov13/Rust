fn main() {
    let num_tup = (13, 7, 94);
    let (x, y, z) = num_tup; //Destructuring

    println!("Tuple values: {}, {}, {}", x, y, z);

    let sum = x + y + z;
    let avg = sum/3;
    let prod = x * y*z;

    println!("The sum is: {}", sum);
    println!("The avg is: {}", avg);
    println!("The prod is: {}", prod);
}
