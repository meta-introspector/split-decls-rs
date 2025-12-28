macro_rules! IntoIter {
    () => {
        # [doc = " An iterator over owned values of type `T`."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub struct IntoIter < T > { inner : vec :: IntoIter < T > , }
    };
}

IntoIter!()