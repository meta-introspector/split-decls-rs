macro_rules! deps {
    () => {
        Punctuated!();
        End!();
    };
}

macro_rules! Pair {
    () => {
        deps!();
        # [doc = " A single syntax tree node of type `T` followed by its trailing punctuation"] # [doc = " of type `P` if any."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub enum Pair < T , P > { Punctuated (T , P) , End (T) , }
    };
}

Pair!();