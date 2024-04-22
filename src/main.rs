use std::time::SystemTime;

fn rng(min: u64, max: u64) -> u64{
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => (n.as_secs() * 3543776743 + 3576876877) % (max-min) + min,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn main() {
    println!("{}", rng(0, 10));
    println!("{}", rng(0, 10));
    println!("{}", rng(0, 10));
    println!("{}", rng(0, 10));
    println!("{}\n", rng(0, 10));

    println!("{}", rng(10, 220));
    println!("{}", rng(0, 10000));
    println!("{}", rng(3, 45));
    println!("{}", rng(0, 24));
    println!("{}", rng(9, 25));
}
