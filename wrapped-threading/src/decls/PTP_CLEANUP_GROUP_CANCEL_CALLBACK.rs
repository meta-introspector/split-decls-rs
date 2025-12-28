macro_rules! PTP_CLEANUP_GROUP_CANCEL_CALLBACK {
    () => {
        pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = Option < unsafe extern "system" fn (objectcontext : * mut core :: ffi :: c_void , cleanupcontext : * mut core :: ffi :: c_void ,) , > ;
    };
}

PTP_CLEANUP_GROUP_CANCEL_CALLBACK!()