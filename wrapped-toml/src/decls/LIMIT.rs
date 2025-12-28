macro_rules! LIMIT {
    () => {
        # [cfg (not (feature = "unbounded"))] const LIMIT : u32 = 80 ;
    };
}

LIMIT!()