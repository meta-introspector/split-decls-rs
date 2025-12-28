macro_rules! make_hash {
    () => {
        # [inline] pub fn make_hash < K : Hash + ? Sized > (val : & K) -> u64 { let mut state = FxHasher :: default () ; val . hash (& mut state) ; state . finish () }
    };
}

make_hash!()