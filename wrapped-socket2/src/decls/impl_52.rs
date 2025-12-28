macro_rules! deps {
    () => {
        Domain!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl From < c_int > for Domain { fn from (d : c_int) -> Domain { Domain (d) } }
    };
}

impl_52!();