macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T > Extend < T > for ThinVec < T > { # [inline] fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = T > , { let iter = iter . into_iter () ; let hint = iter . size_hint () . 0 ; if hint > 0 { self . reserve (hint) ; } for x in iter { self . push (x) ; } } }
    };
}

impl_29!();