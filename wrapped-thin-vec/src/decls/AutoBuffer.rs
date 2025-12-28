macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! AutoBuffer {
    () => {
        deps!();
        # [cfg (feature = "gecko-ffi")] # [repr (C , align (8))] struct AutoBuffer < T , const N : usize > { header : Header , buffer : mem :: MaybeUninit < [T ; N] > , }
    };
}

AutoBuffer!()