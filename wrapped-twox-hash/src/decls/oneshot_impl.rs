macro_rules! deps {
    () => {
        Finalize128!();
    };
}

macro_rules! oneshot_impl {
    () => {
        deps!();
        # [inline] fn oneshot_impl (vector : impl Vector , secret : & Secret , input : & [u8]) -> u128 { Algorithm (vector) . oneshot (secret , input , Finalize128) }
    };
}

oneshot_impl!();