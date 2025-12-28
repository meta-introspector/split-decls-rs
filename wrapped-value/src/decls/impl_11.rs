macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Deref for Name { type Target = str ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_11!()