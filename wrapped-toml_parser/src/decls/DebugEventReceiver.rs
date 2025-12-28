macro_rules! deps {
    () => {
        EventReceiver!();
    };
}

macro_rules! DebugEventReceiver {
    () => {
        deps!();
        pub (crate) struct DebugEventReceiver < 'r > { receiver : & 'r mut dyn crate :: parser :: EventReceiver , }
    };
}

DebugEventReceiver!();