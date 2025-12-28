macro_rules! equivalent_key {
    () => {
        # [inline] fn equivalent_key < K : Eq , V > (k : & K) -> impl Fn (& (K , V)) -> bool + '_ { move | x | x . 0 == * k }
    };
}

equivalent_key!();