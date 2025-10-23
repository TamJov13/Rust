fn main() {
    println!("Hello from main");
    greet();
    greet_person("Tamara");
    print_sum(13, 7);
    increment(13);
    println!("Absolute value of -13: {}", absolute_value(-13));
    println!("Absolute value of 7: {}", absolute_value(7));

    let result = add(13,7);
    println!("Result: {}", result);

    let (q, r) = divide_with_remainder(13,7);
    println!("13 divided by 7 is {} with remainder {}", q, r);

    //statements perform an action but don't return a value
    //expressions evaluate to a value
    let x = 5; //this is statement
    let y = {
        let x = 3;
        x+1 //this is an expression (no semicolon)
    }; //the whole block is an expression 


    let x = {
        let a = 3;
        a+1
    };
    println!("x is: {}", x); //prints: x is 4

    let y = {
        let a = 3;
        a+1; //statement: returns ()
    };
}

fn greet(){
    println!("Hello from the greet function!");
}
//single parameter
fn greet_person(name: &str){
    println!("Hello {}!", name);
}

//multiple parameters
fn print_sum(x: i32, y: i32){
    println!("The sum of {} and {} is {}", x, y, x+y);
}

// If you need to modify, create a mutable variable:
fn increment(x: i32){
    let mut x = x;
    x = x+1;
    println!("Incremented value: {}", x);
}

fn add(x: i32, y: i32) -> i32{
    return x+y;
}

fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x; //early return
    }
    x
}
//returning multiple values with tuples
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}


