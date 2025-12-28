macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl iter :: FromIterator < char > for SmolStr { fn from_iter < I : iter :: IntoIterator < Item = char > > (iter : I) -> SmolStr { from_char_iter (iter . into_iter ()) } }
    };
}

impl_21!();