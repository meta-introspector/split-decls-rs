macro_rules! deps {
    () => {
        Interner!();
        Binder!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < I : Interner , T > Binder < I , Option < T > > { pub fn transpose (self) -> Option < Binder < I , T > > { let Binder { value , bound_vars } = self ; value . map (| value | Binder { value , bound_vars }) } }
    };
}

impl_227!();