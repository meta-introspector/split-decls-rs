macro_rules! accumulator {
    () => {
        # [cfg (feature = "accumulator")] mod accumulator ;
    };
}

accumulator!()