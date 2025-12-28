macro_rules! deps {
    () => {
        RustStringInner!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl RustString { pub fn build_byte_buffer (closure : impl FnOnce (& Self)) -> Vec < u8 > { let buf = RustStringInner :: default () ; closure (buf . as_opaque ()) ; buf . into_inner () } }
    };
}

impl_1!()