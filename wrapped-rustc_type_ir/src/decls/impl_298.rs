macro_rules! deps {
    () => {
        TypeFolder!();
        TypeFoldable!();
        Interner!();
        FallibleTypeFolder!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < I : Interner , T : TypeFoldable < I > , E : TypeFoldable < I > > TypeFoldable < I > for Result < T , E > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { Ok (match self { Ok (v) => Ok (v . try_fold_with (folder) ?) , Err (e) => Err (e . try_fold_with (folder) ?) , }) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { match self { Ok (v) => Ok (v . fold_with (folder)) , Err (e) => Err (e . fold_with (folder)) , } } }
    };
}

impl_298!()