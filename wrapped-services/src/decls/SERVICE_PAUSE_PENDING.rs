macro_rules! deps {
    () => {
        SERVICE_STATUS_CURRENT_STATE!();
    };
}

macro_rules! SERVICE_PAUSE_PENDING {
    () => {
        deps!();
        pub const SERVICE_PAUSE_PENDING : SERVICE_STATUS_CURRENT_STATE = 6u32 ;
    };
}

SERVICE_PAUSE_PENDING!()