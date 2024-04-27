use std::time::SystemTime;

pub fn rng(min: u64, max: u64) -> u128{
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => (n.as_nanos() * 354377617451454163 + 35768279713976877) % ((max-min+1) as u128) + (min as u128),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn rng_seq(min: u64, max: u64, len: usize) -> Vec<u128>{
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

pub fn shuffle<T>(mut arr: Vec<T>) -> Vec<T>{
    for i in 0..arr.len(){
        let j = rng(0, i as u64);
        arr.swap(i, j as usize);
    }  
    arr
}