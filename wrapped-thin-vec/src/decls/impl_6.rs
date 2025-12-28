macro_rules! deps {
    () => {
        UnwrapCapOverflow!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T , E > UnwrapCapOverflow < T > for Result < T , E > { fn unwrap_cap_overflow (self) -> T { match self { Ok (val) => val , Err (_) => capacity_overflow () , } } }
    };
}

impl_6!();