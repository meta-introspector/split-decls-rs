macro_rules! deps {
    () => {
        DebugDepthGuard!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl DebugDepthGuard { pub (crate) fn new () -> Self { let depth = DEBUG_DEPTH . enter_unchecked () ; Self { depth , inc : true } } fn take (& mut self) -> Self { let depth = self . depth ; let inc = self . inc ; self . inc = false ; Self { depth , inc } } }
    };
}

impl_166!()