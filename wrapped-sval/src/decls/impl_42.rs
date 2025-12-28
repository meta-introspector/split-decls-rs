macro_rules! deps {
    () => {
        Value!();
        Tag!();
        Stream!();
        Result!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Value for bool { fn stream < 'sval , S : Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> Result { stream . bool (* self) } fn tag (& self) -> Option < Tag > { None } fn to_bool (& self) -> Option < bool > { Some (* self) } }
    };
}

impl_42!();