macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl From < & str > for SmolStr { # [inline] fn from (s : & str) -> SmolStr { SmolStr :: new (s) } }
    };
}

impl_32!();