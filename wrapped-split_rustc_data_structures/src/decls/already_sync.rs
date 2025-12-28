macro_rules! deps {
    () => {
        DynSync!();
    };
}

macro_rules! already_sync {
    () => {
        deps!();
        macro_rules ! already_sync { ($ ([$ ty : ty]) *) => { $ (unsafe impl DynSync for $ ty where $ ty : Sync { }) * } ; }
    };
}

already_sync!();