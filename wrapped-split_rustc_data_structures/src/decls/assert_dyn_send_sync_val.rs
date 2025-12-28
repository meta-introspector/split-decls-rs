macro_rules! deps {
    () => {
        DynSync!();
        DynSend!();
    };
}

macro_rules! assert_dyn_send_sync_val {
    () => {
        deps!();
        pub fn assert_dyn_send_sync_val < T : ? Sized + PointeeSized + DynSync + DynSend > (_t : & T) { }
    };
}

assert_dyn_send_sync_val!()