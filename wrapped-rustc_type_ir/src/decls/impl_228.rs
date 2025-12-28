macro_rules! deps {
    () => {
        Interner!();
        Binder!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < I : Interner , T : IntoIterator > Binder < I , T > { pub fn iter (self) -> impl Iterator < Item = Binder < I , T :: Item > > { let Binder { value , bound_vars } = self ; value . into_iter () . map (move | value | Binder { value , bound_vars }) } }
    };
}

impl_228!();