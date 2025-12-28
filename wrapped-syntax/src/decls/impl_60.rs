macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl PartialOrd for TokenText < '_ > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_60!();