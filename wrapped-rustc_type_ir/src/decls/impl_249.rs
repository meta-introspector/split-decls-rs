macro_rules! deps {
    () => {
        IterIdentityCopied!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < Iter : IntoIterator > Iterator for IterIdentityCopied < Iter > where Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy , { type Item = < Iter :: Item as Deref > :: Target ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| i | * i) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_249!();