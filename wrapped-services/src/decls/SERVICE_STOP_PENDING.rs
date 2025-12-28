macro_rules! deps {
    () => {
        SERVICE_STATUS_CURRENT_STATE!();
    };
}

macro_rules! SERVICE_STOP_PENDING {
    () => {
        deps!();
        pub const SERVICE_STOP_PENDING : SERVICE_STATUS_CURRENT_STATE = 3u32 ;
    };
}

SERVICE_STOP_PENDING!();