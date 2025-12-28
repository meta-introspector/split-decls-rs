macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl iter :: FromIterator < String > for SmolStr { fn from_iter < I : iter :: IntoIterator < Item = String > > (iter : I) -> SmolStr { build_from_str_iter (iter . into_iter ()) } }
    };
}

impl_25!()