macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl From < & mut str > for SmolStr { # [inline] fn from (s : & mut str) -> SmolStr { SmolStr :: new (s) } }
    };
}

impl_33!();