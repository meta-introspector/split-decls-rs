macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl From < Box < str > > for SmolStr { # [inline] fn from (s : Box < str >) -> SmolStr { SmolStr :: new (s) } }
    };
}

impl_36!()