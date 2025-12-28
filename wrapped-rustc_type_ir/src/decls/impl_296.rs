macro_rules! deps {
    () => {
        TypeFoldable!();
        FallibleTypeFolder!();
        TypeFolder!();
        Interner!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl < I : Interner , A : TypeFoldable < I > , B : TypeFoldable < I > , C : TypeFoldable < I > > TypeFoldable < I > for (A , B , C) { fn try_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F ,) -> Result < (A , B , C) , F :: Error > { Ok ((self . 0 . try_fold_with (folder) ? , self . 1 . try_fold_with (folder) ? , self . 2 . try_fold_with (folder) ? ,)) } fn fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self { (self . 0 . fold_with (folder) , self . 1 . fold_with (folder) , self . 2 . fold_with (folder)) } }
    };
}

impl_296!()