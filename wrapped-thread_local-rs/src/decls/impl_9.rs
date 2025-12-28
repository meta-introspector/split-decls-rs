macro_rules! deps {
    () => {
        CachedIterMut!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'a , T : Send + 'a > Iterator for CachedIterMut < 'a , T > { type Item = & 'a mut T ; # [inline] fn next (& mut self) -> Option < & 'a mut T > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_9!()