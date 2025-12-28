macro_rules! SortPairs {
    () => {
        # [derive (Default)] pub struct SortPairs < K : Ord > { pairs : Vec < (Value , K) > , }
    };
}

SortPairs!()