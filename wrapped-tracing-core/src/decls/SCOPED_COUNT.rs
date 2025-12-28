macro_rules! SCOPED_COUNT {
    () => {
        # [cfg (feature = "std")] static SCOPED_COUNT : AtomicUsize = AtomicUsize :: new (0) ;
    };
}

SCOPED_COUNT!();