macro_rules! deps {
    () => {
        AssocTypeData!();
        AssocItem!();
        AssocKind!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        impl AssocItem { pub fn is_impl_trait_in_trait (& self) -> bool { matches ! (self . kind , AssocKind :: Type { data : AssocTypeData :: Rpitit (_) }) } }
    };
}

impl_454!();