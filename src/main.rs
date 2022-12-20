use rand::prelude::*;
fn main() {
    let mut rng = rand::thread_rng();
    println!("Fixed Random number in f32: {}", rng.gen::<f32>());
}
