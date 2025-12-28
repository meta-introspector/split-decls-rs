macro_rules! deps {
    () => {
        CoreLatch!();
        AsCoreLatch!();
        SpinLatch!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'r > AsCoreLatch for SpinLatch < 'r > { # [inline] fn as_core_latch (& self) -> & CoreLatch { & self . core_latch } }
    };
}

impl_79!();