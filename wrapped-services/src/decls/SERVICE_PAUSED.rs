macro_rules! deps {
    () => {
        SERVICE_STATUS_CURRENT_STATE!();
    };
}

macro_rules! SERVICE_PAUSED {
    () => {
        deps!();
        pub const SERVICE_PAUSED : SERVICE_STATUS_CURRENT_STATE = 7u32 ;
    };
}

SERVICE_PAUSED!()