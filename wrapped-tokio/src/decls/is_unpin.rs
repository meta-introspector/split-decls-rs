macro_rules! is_unpin {
    () => {
        # [cfg (feature = "io-util")] # [cfg (test)] fn is_unpin < T : Unpin > () { }
    };
}

is_unpin!()