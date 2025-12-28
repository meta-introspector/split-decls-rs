macro_rules! other_0 {
    () => {
        # [cfg (all (feature = "alloc" , feature = "rustc-dep-of-std"))] extern crate rustc_std_workspace_alloc as alloc ;
    };
}

other_0!();