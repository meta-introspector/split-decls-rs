macro_rules! deps {
    () => {
        Precedence!();
    };
}

macro_rules! impl_595 {
    () => {
        deps!();
        impl PartialOrd for Precedence { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { let this = * self as u8 ; let other = * other as u8 ; Some (this . cmp (& other)) } }
    };
}

impl_595!()