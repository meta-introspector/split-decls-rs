macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! handle_is_dyn_compatible {
    () => {
        deps!();
        # [test] fn handle_is_dyn_compatible () { fn _assert (_ : & dyn Handle) { } }
    };
}

handle_is_dyn_compatible!();