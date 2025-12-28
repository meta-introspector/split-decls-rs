macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < 'a > From < Cow < 'a , str > > for SmolStr { # [inline] fn from (s : Cow < 'a , str >) -> SmolStr { SmolStr :: new (s) } }
    };
}

impl_38!()