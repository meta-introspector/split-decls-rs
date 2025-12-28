macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 's , T > Extend < T > for SliceVec < 's , T > { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { for t in iter { self . push (t) } } }
    };
}

impl_98!();