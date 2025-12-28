macro_rules! Rng {
    () => {
        trait Rng { fn u128 () -> u128 ; fn u64 () -> u64 ; fn u16 () -> u16 ; }
    };
}

Rng!()