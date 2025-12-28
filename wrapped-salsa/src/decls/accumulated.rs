macro_rules! accumulated {
    () => {
        # [cfg (feature = "accumulator")] mod accumulated ;
    };
}

accumulated!()