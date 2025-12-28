macro_rules! deps {
    () => {
        SERVICE_STATUS_CURRENT_STATE!();
    };
}

macro_rules! SERVICE_START_PENDING {
    () => {
        deps!();
        pub const SERVICE_START_PENDING : SERVICE_STATUS_CURRENT_STATE = 2u32 ;
    };
}

SERVICE_START_PENDING!();