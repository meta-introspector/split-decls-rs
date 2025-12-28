macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! other_10 {
    () => {
        deps!();
        # [cfg (all (feature = "gecko-ffi" , not (test) , not (miri)))] extern "C" { # [link_name = "sEmptyTArrayHeader"] static EMPTY_HEADER : Header ; }
    };
}

other_10!()