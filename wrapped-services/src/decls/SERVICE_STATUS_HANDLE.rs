macro_rules! SERVICE_STATUS_HANDLE {
    () => {
        pub type SERVICE_STATUS_HANDLE = * mut core :: ffi :: c_void ;
    };
}

SERVICE_STATUS_HANDLE!()