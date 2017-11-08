fn main() {
    // print the 5th term in fib sequence
    let x = 6;
    let term = fib(x);
    println!("The {}th term in the fib sequnce is: {}", x, term);
    println!("Beginning recursive func:");
    let term = r_fib(x);
    println!("The {}th term in the fib sequnce is: {}", x, term);
    
}
fn fib(t: i32) -> i32 {
    println!("Beginning non-recursive func:");
    let mut i=1;
    let mut fib = vec![0,1];
    while i<=t {
        let term_i = fib.len() - 1;
        let term_i2 = fib.len() - 2;
        let new_term = fib[term_i] + fib[term_i2];

        fib.push(new_term);

        i = i+1;
    }
    fib[fib.len()-1]
}

fn r_fib(t: i32) -> i32 {
    if t <= 1 {
        1
    } else {
        r_fib(t-1) + r_fib(t-2)
    }
    
}