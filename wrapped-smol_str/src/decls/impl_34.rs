macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl From < & String > for SmolStr { # [inline] fn from (s : & String) -> SmolStr { SmolStr :: new (s) } }
    };
}

impl_34!()