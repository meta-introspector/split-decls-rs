macro_rules! deps {
    () => {
        CycleHeads!();
        CycleHead!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl From < CycleHead > for CycleHeads { fn from (value : CycleHead) -> Self { Self (thin_vec ! [value]) } }
    };
}

impl_64!()