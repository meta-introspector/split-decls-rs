macro_rules! deps {
    () => {
        Extensions!();
        ConstValue!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Deref for Extensions { type Target = HashMap < String , crate :: ConstValue > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_31!()