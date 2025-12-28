macro_rules! deps {
    () => {
        PanicContext!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Drop for PanicContext { fn drop (& mut self) { with_ctx (| ctx | assert ! (ctx . pop () . is_some ())) ; } }
    };
}

impl_33!();