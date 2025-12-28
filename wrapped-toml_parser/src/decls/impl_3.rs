macro_rules! deps {
    () => {
        DebugDepth!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl DebugDepth { pub (crate) fn enter_unchecked (& self) -> usize { self . 0 . fetch_add (1 , core :: sync :: atomic :: Ordering :: SeqCst) } pub (crate) fn exit_unchecked (& self) { let _ = self . 0 . fetch_sub (1 , core :: sync :: atomic :: Ordering :: SeqCst) ; } pub (crate) fn depth (& self) -> usize { self . 0 . load (core :: sync :: atomic :: Ordering :: SeqCst) } }
    };
}

impl_3!();