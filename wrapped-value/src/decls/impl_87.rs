macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Borrow < str > for Name { fn borrow (& self) -> & str { & self . 0 } }
    };
}

impl_87!()