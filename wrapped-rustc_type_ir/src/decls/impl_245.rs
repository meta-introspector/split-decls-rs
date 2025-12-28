macro_rules! deps {
    () => {
        TypeFoldable!();
        EarlyBinder!();
        IterInstantiatedCopied!();
        Interner!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < I : Interner , Iter : IntoIterator > Iterator for IterInstantiatedCopied < '_ , I , Iter > where Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { type Item = < Iter :: Item as Deref > :: Target ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| value | { EarlyBinder { value : * value , _tcx : PhantomData } . instantiate (self . cx , self . args) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_245!()