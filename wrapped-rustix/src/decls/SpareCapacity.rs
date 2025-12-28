macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! SpareCapacity {
    () => {
        deps!();
        # [doc = " A type that implements [`Buffer`] by appending to a `Vec`, up to its"] # [doc = " capacity."] # [doc = ""] # [doc = " To use this, use the [`spare_capacity`] function."] # [doc = ""] # [doc = " Because this uses the capacity, and never reallocates, the `Vec` should"] # [doc = " have some non-empty spare capacity."] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub struct SpareCapacity < 'a , T > (& 'a mut Vec < T >) ;
    };
}

SpareCapacity!()