macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl From < c_int > for Type { fn from (t : c_int) -> Type { Type (t) } }
    };
}

impl_56!()