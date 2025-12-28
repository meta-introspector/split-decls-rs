macro_rules! nonmax {
    () => {
        # [cfg (feature = "nonmax")] mod nonmax ;
    };
}

nonmax!();