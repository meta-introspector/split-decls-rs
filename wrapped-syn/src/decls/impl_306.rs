macro_rules! deps {
    () => {
        ConstParams!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl < 'a > Iterator for ConstParams < 'a > { type Item = & 'a ConstParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Const (const_param) = self . 0 . next () ? { Some (const_param) } else { self . next () } } }
    };
}

impl_306!();