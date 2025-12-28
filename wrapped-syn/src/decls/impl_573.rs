macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_573 {
    () => {
        deps!();
        impl < T > From < T > for Path where T : Into < PathSegment > , { fn from (segment : T) -> Self { let mut path = Path { leading_colon : None , segments : Punctuated :: new () , } ; path . segments . push_value (segment . into ()) ; path } }
    };
}

impl_573!()