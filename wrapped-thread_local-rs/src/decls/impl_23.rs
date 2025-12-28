macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'a , T : Send > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < & 'a mut T > { self . raw . next_mut (self . thread_local) . map (| entry | unsafe { (& mut * entry . value . get ()) . assume_init_mut () }) } fn size_hint (& self) -> (usize , Option < usize >) { self . raw . size_hint_frozen (self . thread_local) } }
    };
}

impl_23!()