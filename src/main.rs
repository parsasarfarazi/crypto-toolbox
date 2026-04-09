use crypto_toolbox::bignum::biguint::*;

fn main() {
    let mut a = U2048::new();
    a = a + U2048::from(1_u8);

    println!("result is {:?}",a)
}