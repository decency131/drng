```pub fn rng(min: u64, max: u64) -> u128```

returns a number between ```min``` and ```max```

```pub fn rng_seq(min: u64, max: u64, len: usize) -> Box<dyn Iterator<Item = u128>>```

returns an iterator over a sequence of numbers between ```min``` and ```max``` of lenghth ```len```

```pub trait Shuffle```

currently implemented for Arrays and Vectors. shuffles what it's called on.


For better understanding of usage see [eaxmples](https://github.com/decency131/drng/blob/998f82523c04487f507354caf0700fdfbe70c805/examples/example0.rs).
