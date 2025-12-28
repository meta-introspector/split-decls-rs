macro_rules! deps {
    () => {
        BoolExt!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl BoolExt for bool { fn then_some < T > (self , t : T) -> Option < T > { if self { Some (t) } else { None } } }
    };
}

impl_64!()