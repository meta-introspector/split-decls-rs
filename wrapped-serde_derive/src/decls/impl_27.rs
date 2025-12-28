macro_rules! deps {
    () => {
        Field!();
        Identifier!();
        Variant!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Identifier { # [cfg (feature = "deserialize_in_place")] pub fn is_some (self) -> bool { match self { Identifier :: No => false , Identifier :: Field | Identifier :: Variant => true , } } }
    };
}

impl_27!()