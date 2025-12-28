macro_rules! deps {
    () => {
        EdgesVec!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl FromIterator < DepNodeIndex > for EdgesVec { # [inline] fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = DepNodeIndex > , { let mut vec = EdgesVec :: new () ; for index in iter { vec . push (index) } vec } }
    };
}

impl_37!()