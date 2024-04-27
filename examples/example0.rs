extern crate drng;
fn main() {
    println!("{}\n", drng::rng(0, 10));

    println!("{:#?}\n", drng::rng_seq(0, 10, 5));

    let mut vec = Vec::with_capacity(10);
    for i in 0..10{ vec.push(i);}

    println!("{:#?}", vec);
    println!("{:#?}", drng::shuffle(vec));
}
