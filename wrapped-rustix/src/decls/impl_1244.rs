macro_rules! deps {
    () => {
        WaitPtr!();
    };
}

macro_rules! impl_1244 {
    () => {
        deps!();
        impl From < * mut c_void > for WaitPtr { # [inline] fn from (ptr : * mut c_void) -> Self { Self :: new (ptr) } }
    };
}

impl_1244!()