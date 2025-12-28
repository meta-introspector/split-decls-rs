macro_rules! deps {
    () => {
        IntoIter!();
        Iter!();
    };
}

macro_rules! Pairs {
    () => {
        deps!();
        # [doc = " An iterator over borrowed pairs of type `Pair<&T, &P>`."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub struct Pairs < 'a , T : 'a , P : 'a > { inner : slice :: Iter < 'a , (T , P) > , last : option :: IntoIter < & 'a T > , }
    };
}

Pairs!();