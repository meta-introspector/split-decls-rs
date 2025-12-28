macro_rules! deps {
    () => {
        SlotIndex!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl SlotIndex { # [inline] fn new (idx : usize) -> Self { debug_assert ! (idx < PAGE_LEN) ; Self (idx) } }
    };
}

impl_329!();