macro_rules! deps {
    () => {
        TypeFolder!();
        FallibleTypeFolder!();
        Interner!();
        TypeFoldable!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < I : Interner , T : TypeFoldable < I > > TypeFoldable < I > for Arc < T > { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F) -> Result < Self , F :: Error > { fold_arc (self , | t | t . try_fold_with (folder)) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { match fold_arc :: < T , Infallible > (self , | t | Ok (t . fold_with (folder))) { Ok (t) => t , } } }
    };
}

impl_300!()