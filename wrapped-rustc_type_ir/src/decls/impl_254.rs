macro_rules! deps {
    () => {
        Interner!();
        EarlyBinderIter!();
        EarlyBinder!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl < I : Interner , T : Iterator > Iterator for EarlyBinderIter < I , T > { type Item = EarlyBinder < I , T :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . t . next () . map (| value | EarlyBinder { value , _tcx : PhantomData }) } fn size_hint (& self) -> (usize , Option < usize >) { self . t . size_hint () } }
    };
}

impl_254!()