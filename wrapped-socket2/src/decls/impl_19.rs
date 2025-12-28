macro_rules! deps {
    () => {
        Protocol!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl From < Protocol > for c_int { fn from (p : Protocol) -> c_int { p . 0 } }
    };
}

impl_19!()