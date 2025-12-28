macro_rules! deps {
    () => {
        TypeFoldable!();
        Binder!();
        Interner!();
        TypeSuperFoldable!();
        TypeFolder!();
        FallibleTypeFolder!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < I : Interner , T : TypeFoldable < I > > TypeSuperFoldable < I > for Binder < I , T > { fn try_super_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F ,) -> Result < Self , F :: Error > { self . try_map_bound (| t | t . try_fold_with (folder)) } fn super_fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { self . map_bound (| t | t . fold_with (folder)) } }
    };
}

impl_224!()