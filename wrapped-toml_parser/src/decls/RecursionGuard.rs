macro_rules! deps {
    () => {
        EventReceiver!();
    };
}

macro_rules! RecursionGuard {
    () => {
        deps!();
        pub struct RecursionGuard < 'r > { receiver : & 'r mut dyn EventReceiver , max_depth : u32 , depth : i64 , }
    };
}

RecursionGuard!()