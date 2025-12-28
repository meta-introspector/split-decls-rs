macro_rules! deps {
    () => {
        Variables!();
        ConstValue!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl From < Variables > for ConstValue { fn from (variables : Variables) -> Self { variables . into_value () } }
    };
}

impl_79!()