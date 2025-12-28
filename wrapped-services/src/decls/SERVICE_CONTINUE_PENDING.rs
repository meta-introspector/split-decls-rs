macro_rules! deps {
    () => {
        SERVICE_STATUS_CURRENT_STATE!();
    };
}

macro_rules! SERVICE_CONTINUE_PENDING {
    () => {
        deps!();
        pub const SERVICE_CONTINUE_PENDING : SERVICE_STATUS_CURRENT_STATE = 5u32 ;
    };
}

SERVICE_CONTINUE_PENDING!()