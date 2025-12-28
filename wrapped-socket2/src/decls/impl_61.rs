macro_rules! deps {
    () => {
        Protocol!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl From < Protocol > for c_int { fn from (p : Protocol) -> c_int { p . 0 } }
    };
}

impl_61!();