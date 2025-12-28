macro_rules! deps {
    () => {
        TypeFoldable!();
        FallibleTypeFolder!();
        TypeFolder!();
        Interner!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < I : Interner , T : TypeFoldable < I > > TypeFoldable < I > for Option < T > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { Ok (match self { Some (v) => Some (v . try_fold_with (folder) ?) , None => None , }) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { Some (self ? . fold_with (folder)) } }
    };
}

impl_297!()