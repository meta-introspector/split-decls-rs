macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < 'a , T : Send + Sync > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next (self . thread_local) } fn size_hint (& self) -> (usize , Option < usize >) { self . raw . size_hint (self . thread_local) } }
    };
}

impl_41!();