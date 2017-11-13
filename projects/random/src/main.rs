extern crate rand;

use rand::distributions::{IndependentSample, Range};

fn main() {
    let between = Range::new(5,11);
    let mut rng = rand::thread_rng();

    for _ in 1..100 {

        let x = between.ind_sample(&mut rng);
        let y = between.ind_sample(&mut rng);
        println!("random number1: {}", x as u16);
        println!("random number2: {}", y as u16)
    
    }
}
