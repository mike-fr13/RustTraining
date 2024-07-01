fn main() {
    // an example o recusivity with a function gcd that calculate the greatest common divisor of two numbers using a recusrsive function gcd
    // the function gcd is defined as follows:  
    // gcd(a, b) = a if b = 0
    // gcd(a, b) = gcd(b, a % b) otherwise
    // the function gcd is called with two numbers a and b
    // the function gcd returns the greatest common divisor of a and b
    // the function gcd is called with two numbers a and b
    // the function gcd returns the greatest common divisor of a and b

    let a = 48;
    let b = 18; 
    let result = gcd(a, b);
    println!("gcd({},{}) = {}", a, b, result);
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}
