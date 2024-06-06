extern crate drng;
use drng::Shuffle;

fn main() {
    println!("{}\n", drng::rng(0, 10));

    for i in drng::rng_seq(0, 10, 5) {
        println! ("{} ", i);
    }

    let mut vec = Vec::with_capacity(10);
    for i in 0..10{ vec.push(i);}

    println!("{:#?}", vec);
    println!("{:#?}", vec.shuffle());

    println!("{:#?}", [0,1,2,3,4,5,6,7,8,9].shuffle());
}
