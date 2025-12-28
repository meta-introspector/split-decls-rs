macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Borrow < str > for Name { fn borrow (& self) -> & str { & self . 0 } }
    };
}

impl_10!()