macro_rules! deps {
    () => {
        SERVICE_STATUS_CURRENT_STATE!();
    };
}

macro_rules! SERVICE_STOPPED {
    () => {
        deps!();
        pub const SERVICE_STOPPED : SERVICE_STATUS_CURRENT_STATE = 1u32 ;
    };
}

SERVICE_STOPPED!();