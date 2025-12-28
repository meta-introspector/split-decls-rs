macro_rules! deps {
    () => {
        Protocol!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl From < c_int > for Protocol { fn from (p : c_int) -> Protocol { Protocol (p) } }
    };
}

impl_60!();