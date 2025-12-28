macro_rules! deps {
    () => {
        RecursionGuard!();
        EventReceiver!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < 'r > RecursionGuard < 'r > { pub fn new (receiver : & 'r mut dyn EventReceiver , max_depth : u32) -> Self { Self { receiver , max_depth , depth : 0 , } } fn within_depth (& self) -> bool { self . depth <= self . max_depth as i64 } }
    };
}

impl_187!();