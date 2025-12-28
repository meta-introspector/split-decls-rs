macro_rules! deps {
    () => {
        TypeParams!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < 'a > Iterator for TypeParams < 'a > { type Item = & 'a TypeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Type (type_param) = self . 0 . next () ? { Some (type_param) } else { self . next () } } }
    };
}

impl_302!()