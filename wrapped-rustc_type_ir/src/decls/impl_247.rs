macro_rules! deps {
    () => {
        IterInstantiatedCopied!();
        Interner!();
        TypeFoldable!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl < I : Interner , Iter : IntoIterator > ExactSizeIterator for IterInstantiatedCopied < '_ , I , Iter > where Iter :: IntoIter : ExactSizeIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy + TypeFoldable < I > , { }
    };
}

impl_247!()