macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl From < Type > for c_int { fn from (t : Type) -> c_int { t . 0 } }
    };
}

impl_57!();