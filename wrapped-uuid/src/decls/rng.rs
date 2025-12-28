macro_rules! rng {
    () => {
        # [cfg (feature = "rng")] mod rng ;
    };
}

rng!()