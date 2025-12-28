macro_rules! deps {
    () => {
        Uuid!();
        NonNilUuid!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl PartialEq < NonNilUuid > for Uuid { fn eq (& self , other : & NonNilUuid) -> bool { * self == other . get () } }
    };
}

impl_25!();