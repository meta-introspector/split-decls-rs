macro_rules! deps {
    () => {
        CoreLatch!();
    };
}

macro_rules! AsCoreLatch {
    () => {
        deps!();
        pub (super) trait AsCoreLatch { fn as_core_latch (& self) -> & CoreLatch ; }
    };
}

AsCoreLatch!()