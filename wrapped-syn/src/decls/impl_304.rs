macro_rules! deps {
    () => {
        TypeParamsMut!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < 'a > Iterator for TypeParamsMut < 'a > { type Item = & 'a mut TypeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Type (type_param) = self . 0 . next () ? { Some (type_param) } else { self . next () } } }
    };
}

impl_304!()