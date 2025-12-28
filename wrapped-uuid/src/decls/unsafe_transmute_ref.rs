macro_rules! unsafe_transmute_ref {
    () => {
        # [cfg (all (uuid_unstable , feature = "zerocopy"))] macro_rules ! unsafe_transmute_ref (($ e : expr) => { zerocopy :: transmute_ref ! ($ e) }) ;
    };
}

unsafe_transmute_ref!()