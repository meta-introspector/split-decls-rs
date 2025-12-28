macro_rules! deps {
    () => {
        CachedIntoIter!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T : Send > Iterator for CachedIntoIter < T > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_12!()