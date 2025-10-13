fn main() {
    //mutable variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; 
    println!("The value of x is: {}", x);
    //example when use mutability
    let mut count = 0;
    for i in 1..=5{
        count +=i;
    }
    println!("The sum is: {}", count);

    //constants 
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);

    //shadowing
    let y = 5;
    let y = y+1; //shadow y with a new value
    {
        let y=y*2; //shadow y again in this inner scope
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    let spaces= "   ";
    let spaces = spaces.len();
    println!("Len of spaces is: {}", spaces);

    let input = "42";
    println!("Input as string: {}", input);

    let input = input.parse::<i32>().unwrap();
    println!("Input as number: {}", input);

    let input = input * 2;
    println!("Input doubled: {}", input);

    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The tuple is: ({}, {}, {})", tup.0, tup.1, tup.2);

    //destructuring tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; //Destructuring

    println!("The value of y is {}", y);

    let unit = ();
    println!("Unit type: {:?}", unit);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // [type; length]
    
    println!("Array: {:?}", a);

    
}
