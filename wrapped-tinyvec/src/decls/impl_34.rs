macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < A : Array > Extend < A :: Item > for ArrayVec < A > { # [inline] fn extend < T : IntoIterator < Item = A :: Item > > (& mut self , iter : T) { for t in iter { self . push (t) } } }
    };
}

impl_34!()