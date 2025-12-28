macro_rules! unsafe_transmute {
    () => {
        # [cfg (all (uuid_unstable , feature = "zerocopy"))] macro_rules ! unsafe_transmute (($ e : expr) => { zerocopy :: transmute ! ($ e) }) ;
    };
}

unsafe_transmute!();