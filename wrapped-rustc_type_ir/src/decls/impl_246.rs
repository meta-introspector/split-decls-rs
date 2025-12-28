macro_rules! deps {
    () => {
        TypeFoldable!();
        EarlyBinder!();
        IterInstantiatedCopied!();
        Interner!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < I : Interner , Iter : IntoIterator > DoubleEndedIterator for IterInstantiatedCopied < '_ , I , Iter > where Iter :: IntoIter : DoubleEndedIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (| value | { EarlyBinder { value : * value , _tcx : PhantomData } . instantiate (self . cx , self . args) }) } }
    };
}

impl_246!();