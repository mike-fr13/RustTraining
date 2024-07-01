fn main() {

    // create 3 examples of loops from 1 to 5 that display the counter using loop, while, and iter
    // loop 
    let mut i = 1;
    loop {
        println!("loop: i = {}", i);
        i += 1;
        if i > 5 {
            break;
        }
    }
    // a loop with while from 1 to 5 that displays the counter
    i = 1;  // we reset i    
    while i <= 5 {
        println!("while: i = {}", i);
        i += 1;
    }

    // a loop with iter from 1 to 5 that displays the counter
    for i in 1..6 {
        println!("iter: i = {}", i);
    }

    // one example with iter_mut
    let mut v = vec![1, 2, 3, 4, 5];
    for i in v.iter_mut() {
        *i += 1;
    }
    
    
}
