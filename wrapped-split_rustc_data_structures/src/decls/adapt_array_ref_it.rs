macro_rules! adapt_array_ref_it {
    () => {
        # [doc = " adapts Item of array reference iterator to Item of hashmap reference iterator."] # [inline (always)] fn adapt_array_ref_it < K , V > (pair : & (K , V)) -> (& K , & V) { let (a , b) = pair ; (a , b) }
    };
}

adapt_array_ref_it!();