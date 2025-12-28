macro_rules! deps {
    () => {
        AsCoreLatch!();
        CoreLatch!();
        OnceLatch!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl AsCoreLatch for OnceLatch { # [inline] fn as_core_latch (& self) -> & CoreLatch { & self . core_latch } }
    };
}

impl_86!()