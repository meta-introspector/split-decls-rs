macro_rules! deps {
    () => {
        DynSend!();
    };
}

macro_rules! assert_dyn_send {
    () => {
        deps!();
        pub fn assert_dyn_send < T : ? Sized + PointeeSized + DynSend > () { }
    };
}

assert_dyn_send!()