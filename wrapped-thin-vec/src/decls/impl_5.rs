macro_rules! deps {
    () => {
        UnwrapCapOverflow!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T > UnwrapCapOverflow < T > for Option < T > { fn unwrap_cap_overflow (self) -> T { match self { Some (val) => val , None => capacity_overflow () , } } }
    };
}

impl_5!();