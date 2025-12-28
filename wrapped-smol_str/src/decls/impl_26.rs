macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a > iter :: FromIterator < & 'a String > for SmolStr { fn from_iter < I : iter :: IntoIterator < Item = & 'a String > > (iter : I) -> SmolStr { SmolStr :: from_iter (iter . into_iter () . map (| x | x . as_str ())) } }
    };
}

impl_26!();