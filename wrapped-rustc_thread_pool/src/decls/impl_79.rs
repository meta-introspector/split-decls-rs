macro_rules! deps {
    () => {
        AsCoreLatch!();
        SpinLatch!();
        CoreLatch!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'r > AsCoreLatch for SpinLatch < 'r > { # [inline] fn as_core_latch (& self) -> & CoreLatch { & self . core_latch } }
    };
}

impl_79!()