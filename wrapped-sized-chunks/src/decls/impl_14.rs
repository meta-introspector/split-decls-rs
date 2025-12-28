macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < A , T > Drop for InlineArray < A , T > { fn drop (& mut self) { unsafe { self . drop_contents () } } }
    };
}

impl_14!();