macro_rules! deps {
    () => {
        DynSync!();
    };
}

macro_rules! impl_dyn_sync {
    () => {
        deps!();
        macro_rules ! impl_dyn_sync { ($ ($ ($ attr : meta) * [$ ty : ty where $ ($ generics2 : tt) *]) *) => { $ (unsafe impl <$ ($ generics2) *> DynSync for $ ty { }) * } ; }
    };
}

impl_dyn_sync!()