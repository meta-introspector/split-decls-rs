macro_rules! deps {
    () => {
        IterIdentityCopied!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl < Iter : IntoIterator > ExactSizeIterator for IterIdentityCopied < Iter > where Iter :: IntoIter : ExactSizeIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy , { }
    };
}

impl_251!()