macro_rules! deps {
    () => {
        DynSend!();
    };
}

macro_rules! already_send {
    () => {
        deps!();
        macro_rules ! already_send { ($ ([$ ty : ty]) *) => { $ (unsafe impl DynSend for $ ty where $ ty : Send { }) * } ; }
    };
}

already_send!();