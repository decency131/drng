extern crate drng;
fn main() {
    println!("{}\n", drng::rng(0, 10));

    println!("{:#?}\n", drng::rng_seq(0, 10, 5));
}
