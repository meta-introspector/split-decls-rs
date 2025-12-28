macro_rules! deps {
    () => {
        TP_CALLBACK_ENVIRON_V3_0_0!();
    };
}

macro_rules! other_18 {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub union TP_CALLBACK_ENVIRON_V3_0 { pub Flags : u32 , pub s : TP_CALLBACK_ENVIRON_V3_0_0 , }
    };
}

other_18!()