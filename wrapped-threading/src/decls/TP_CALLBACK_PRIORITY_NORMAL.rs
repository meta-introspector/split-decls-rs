macro_rules! deps {
    () => {
        TP_CALLBACK_PRIORITY!();
    };
}

macro_rules! TP_CALLBACK_PRIORITY_NORMAL {
    () => {
        deps!();
        pub const TP_CALLBACK_PRIORITY_NORMAL : TP_CALLBACK_PRIORITY = 1i32 ;
    };
}

TP_CALLBACK_PRIORITY_NORMAL!()