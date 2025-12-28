macro_rules! deps {
    () => {
        SockRef!();
        Socket!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < 's > Deref for SockRef < 's > { type Target = Socket ; fn deref (& self) -> & Self :: Target { & self . socket } }
    };
}

impl_43!()