macro_rules! deps {
    () => {
        EventReceiver!();
        DebugEventReceiver!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'r > DebugEventReceiver < 'r > { pub (crate) fn new (receiver : & 'r mut dyn crate :: parser :: EventReceiver) -> Self { Self { receiver } } }
    };
}

impl_10!();