macro_rules! deps {
    () => {
        ConstParamsMut!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < 'a > Iterator for ConstParamsMut < 'a > { type Item = & 'a mut ConstParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Const (const_param) = self . 0 . next () ? { Some (const_param) } else { self . next () } } }
    };
}

impl_308!()