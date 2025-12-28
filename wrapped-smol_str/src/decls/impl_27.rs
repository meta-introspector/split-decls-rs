macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'a > iter :: FromIterator < & 'a str > for SmolStr { fn from_iter < I : iter :: IntoIterator < Item = & 'a str > > (iter : I) -> SmolStr { build_from_str_iter (iter . into_iter ()) } }
    };
}

impl_27!();