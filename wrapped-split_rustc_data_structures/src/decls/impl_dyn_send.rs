macro_rules! deps {
    () => {
        DynSend!();
    };
}

macro_rules! impl_dyn_send {
    () => {
        deps!();
        macro_rules ! impl_dyn_send { ($ ($ ($ attr : meta) * [$ ty : ty where $ ($ generics2 : tt) *]) *) => { $ (unsafe impl <$ ($ generics2) *> DynSend for $ ty { }) * } ; }
    };
}

impl_dyn_send!()