use crypto_toolbox::prng;

fn main() {
    println!("the odd number is {:?}" , prng::odd_new());
    println!("the random number is {:?}" , prng::new());
}