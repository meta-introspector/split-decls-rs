macro_rules! deps {
    () => {
        CallOnDrop!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < O , F : FnOnce () -> O > Drop for CallOnDrop < O , F > { fn drop (& mut self) { let f = unsafe { ManuallyDrop :: take (& mut self . f) } ; f () ; } }
    };
}

impl_85!();