macro_rules! deps {
    () => {
        Allocator!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        unsafe impl Sync for Allocator < 'static > { }
    };
}

impl_27!()