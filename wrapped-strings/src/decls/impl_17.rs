macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Drop for BSTR { fn drop (& mut self) { if ! self . 0 . is_null () { unsafe { bindings :: SysFreeString (self . 0) } } } }
    };
}

impl_17!()