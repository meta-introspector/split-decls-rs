macro_rules! deps {
    () => {
        CoreLatch!();
        AsCoreLatch!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl AsCoreLatch for CoreLatch { # [inline] fn as_core_latch (& self) -> & CoreLatch { self } }
    };
}

impl_76!();