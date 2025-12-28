macro_rules! deps {
    () => {
        PageIndex!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl PageIndex { # [inline] fn new (idx : usize) -> Self { debug_assert ! (idx < MAX_PAGES) ; Self (idx) } # [allow (dead_code)] pub fn as_usize (& self) -> usize { self . 0 } }
    };
}

impl_327!();