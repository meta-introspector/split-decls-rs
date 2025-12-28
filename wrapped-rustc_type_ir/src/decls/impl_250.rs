macro_rules! deps {
    () => {
        IterIdentityCopied!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl < Iter : IntoIterator > DoubleEndedIterator for IterIdentityCopied < Iter > where Iter :: IntoIter : DoubleEndedIterator , Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (| i | * i) } }
    };
}

impl_250!();