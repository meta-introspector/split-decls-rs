macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl core :: ops :: AddAssign < usize > for Span { fn add_assign (& mut self , rhs : usize) { self . start += rhs ; self . end += rhs ; } }
    };
}

impl_33!()