macro_rules! VERBOSE {
    () => {
        # [cfg (feature = "std")] pub (crate) const VERBOSE : bool = false ;
    };
}

VERBOSE!()