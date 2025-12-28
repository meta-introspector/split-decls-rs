macro_rules! deps {
    () => {
        LifetimesMut!();
        Lifetime!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < 'a > Iterator for LifetimesMut < 'a > { type Item = & 'a mut LifetimeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Lifetime (lifetime) = self . 0 . next () ? { Some (lifetime) } else { self . next () } } }
    };
}

impl_300!();