macro_rules! deps {
    () => {
        TypeFoldable!();
        Interner!();
        FallibleTypeFolder!();
        TypeFolder!();
        Binder!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < I : Interner , T : TypeFoldable < I > > TypeFoldable < I > for Binder < I , T > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { folder . try_fold_binder (self) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { folder . fold_binder (self) } }
    };
}

impl_222!()