macro_rules! deps {
    () => {
        SERVICE_STATUS_CURRENT_STATE!();
    };
}

macro_rules! SERVICE_RUNNING {
    () => {
        deps!();
        pub const SERVICE_RUNNING : SERVICE_STATUS_CURRENT_STATE = 4u32 ;
    };
}

SERVICE_RUNNING!();