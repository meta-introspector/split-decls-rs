macro_rules! deps {
    () => {
        ConstValue!();
        Name!();
        Variables!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl Deref for Variables { type Target = BTreeMap < Name , ConstValue > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_80!()