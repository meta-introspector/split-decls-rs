macro_rules! deps {
    () => {
        ENUM_SERVICE_TYPE!();
    };
}

macro_rules! SERVICE_WIN32_OWN_PROCESS {
    () => {
        deps!();
        pub const SERVICE_WIN32_OWN_PROCESS : ENUM_SERVICE_TYPE = 16u32 ;
    };
}

SERVICE_WIN32_OWN_PROCESS!()