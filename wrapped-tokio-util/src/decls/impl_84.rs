macro_rules! deps {
    () => {
        CallOnDrop!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < O , F : FnOnce () -> O > CallOnDrop < O , F > { fn new (f : F) -> Self { let f = ManuallyDrop :: new (f) ; Self { f } } fn call (self) -> O { let mut this = ManuallyDrop :: new (self) ; let f = unsafe { ManuallyDrop :: take (& mut this . f) } ; f () } }
    };
}

impl_84!();