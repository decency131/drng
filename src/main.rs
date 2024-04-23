use std::time::SystemTime;

fn rng(min: u64, max: u64) -> u64{
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => (n.as_secs() * 3543776743 + 3576876877) % (max-min+1 ) + min,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn rng_seq(min: u64, max: u64, len: usize) -> Vec<u64>{
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let mut seq: Vec<u64> = Vec::with_capacity(len);
            let mut x = n.as_secs();
            for _i in 0..len {
                x = (x * 3543776743 + 3576876877) % (max-min+1 ) + min;
                seq.push(x);
            }
            seq
        },
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn main() {
    println!("{}", rng(0, 10));
    println!("{}", rng(0, 10));
    println!("{}", rng(0, 10));
    println!("{}", rng(0, 10));
    println!("{}\n", rng(0, 10));

    println!("{:#?}\n", rng_seq(0, 10, 5));

    println!("{}", rng(10, 220));
    println!("{}", rng(0, 10000));
    println!("{}", rng(3, 45));
    println!("{}", rng(0, 24));
    println!("{}", rng(9, 25));
}
