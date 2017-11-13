extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    for _ in 1..100 {
        let rand_num = rand::random::<u8>;
        println!("random number: {}", rand_num as u16);
    
    }
}
