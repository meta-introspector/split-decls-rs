macro_rules! deps {
    () => {
        Result!();
        Value!();
        Stream!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl Value for () { fn stream < 'sval , S : Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> Result { stream . tag (Some (& tags :: RUST_UNIT) , None , None) } }
    };
}

impl_41!();