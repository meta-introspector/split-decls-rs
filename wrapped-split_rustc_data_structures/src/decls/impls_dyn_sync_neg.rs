macro_rules! deps {
    () => {
        DynSync!();
    };
}

macro_rules! impls_dyn_sync_neg {
    () => {
        deps!();
        macro_rules ! impls_dyn_sync_neg { ($ ([$ t1 : ty $ (where $ ($ generics1 : tt) *) ?]) *) => { $ (impl $ (<$ ($ generics1) *>) ? ! DynSync for $ t1 { }) * } ; }
    };
}

impls_dyn_sync_neg!();