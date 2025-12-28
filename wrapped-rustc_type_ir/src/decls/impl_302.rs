macro_rules! deps {
    () => {
        TypeFolder!();
        Interner!();
        FallibleTypeFolder!();
        TypeFoldable!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < I : Interner , T : TypeFoldable < I > > TypeFoldable < I > for Vec < T > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { self . into_iter () . map (| t | t . try_fold_with (folder)) . collect () } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { self . into_iter () . map (| t | t . fold_with (folder)) . collect () } }
    };
}

impl_302!()