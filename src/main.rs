use std::time::SystemTime;

fn rng(min: u64, max: u64) -> u128{
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => (n.as_nanos() * 354377617451454163 + 35768279713976877) % ((max-min+1) as u128) + (min as u128),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn rng_seq(min: u64, max: u64, len: usize) -> Vec<u128>{
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let mut seq: Vec<u128> = Vec::with_capacity(len);
            let mut x = n.as_nanos();
            for _i in 0..len {
                x = (x * 354377617451454163 + 35768279713976877) % ((max-min+1) as u128) + (min as u128);
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
