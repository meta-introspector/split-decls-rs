macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! IntoPairs {
    () => {
        deps!();
        # [doc = " An iterator over owned pairs of type `Pair<T, P>`."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub struct IntoPairs < T , P > { inner : vec :: IntoIter < (T , P) > , last : option :: IntoIter < T > , }
    };
}

IntoPairs!()