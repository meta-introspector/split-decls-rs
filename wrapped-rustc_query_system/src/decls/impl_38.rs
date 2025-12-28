macro_rules! deps {
    () => {
        EdgesVec!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl Extend < DepNodeIndex > for EdgesVec { # [inline] fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = DepNodeIndex > , { for elem in iter { self . push (elem) ; } } }
    };
}

impl_38!()