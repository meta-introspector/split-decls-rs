macro_rules! deps {
    () => {
        Domain!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < Domain > for c_int { fn from (d : Domain) -> c_int { d . 0 } }
    };
}

impl_11!()