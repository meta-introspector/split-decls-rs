macro_rules! deps {
    () => {
        FallibleTypeFolder!();
        TypeFoldable!();
        Interner!();
        TypeFolder!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < I : Interner , T : TypeFoldable < I > > TypeFoldable < I > for Box < [T] > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { Vec :: from (self) . try_fold_with (folder) . map (Vec :: into_boxed_slice) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { Vec :: into_boxed_slice (Vec :: from (self) . fold_with (folder)) } }
    };
}

impl_304!()