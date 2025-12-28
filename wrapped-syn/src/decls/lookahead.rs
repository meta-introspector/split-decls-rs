macro_rules! lookahead {
    () => {
        # [cfg (feature = "parsing")] mod lookahead ;
    };
}

lookahead!()