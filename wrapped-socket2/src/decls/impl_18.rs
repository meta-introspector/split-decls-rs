macro_rules! deps {
    () => {
        Protocol!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl From < c_int > for Protocol { fn from (p : c_int) -> Protocol { Protocol (p) } }
    };
}

impl_18!()