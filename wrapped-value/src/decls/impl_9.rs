macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl AsRef < str > for Name { fn as_ref (& self) -> & str { & self . 0 } }
    };
}

impl_9!()