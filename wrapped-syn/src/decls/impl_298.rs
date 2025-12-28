macro_rules! deps {
    () => {
        Lifetimes!();
        Lifetime!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < 'a > Iterator for Lifetimes < 'a > { type Item = & 'a LifetimeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Lifetime (lifetime) = self . 0 . next () ? { Some (lifetime) } else { self . next () } } }
    };
}

impl_298!();