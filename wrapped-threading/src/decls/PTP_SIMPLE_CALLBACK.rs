macro_rules! deps {
    () => {
        PTP_CALLBACK_INSTANCE!();
    };
}

macro_rules! PTP_SIMPLE_CALLBACK {
    () => {
        deps!();
        pub type PTP_SIMPLE_CALLBACK = Option < unsafe extern "system" fn (instance : PTP_CALLBACK_INSTANCE , context : * mut core :: ffi :: c_void) , > ;
    };
}

PTP_SIMPLE_CALLBACK!();