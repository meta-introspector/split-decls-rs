macro_rules! spin {
    () => {
        # [cfg (not (feature = "std"))] pub (crate) mod spin ;
    };
}

spin!();