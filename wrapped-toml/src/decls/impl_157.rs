macro_rules! deps {
    () => {
        Deserializer!();
        DeTable!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'i > From < Spanned < DeTable < 'i > > > for Deserializer < 'i > { fn from (root : Spanned < DeTable < 'i > >) -> Self { let span = root . span () ; let root = root . into_inner () ; Self { span , root , raw : None , } } }
    };
}

impl_157!();