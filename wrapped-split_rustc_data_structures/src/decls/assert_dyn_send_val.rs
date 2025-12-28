macro_rules! deps {
    () => {
        DynSend!();
    };
}

macro_rules! assert_dyn_send_val {
    () => {
        deps!();
        pub fn assert_dyn_send_val < T : ? Sized + PointeeSized + DynSend > (_t : & T) { }
    };
}

assert_dyn_send_val!();