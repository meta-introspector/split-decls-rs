macro_rules! deps {
    () => {
        TypeFoldable!();
        TypeFolder!();
        Interner!();
        FallibleTypeFolder!();
    };
}

macro_rules! TypeSuperFoldable {
    () => {
        deps!();
        pub trait TypeSuperFoldable < I : Interner > : TypeFoldable < I > { # [doc = " Provides a default fold for a recursive type of interest. This should"] # [doc = " only be called within `TypeFolder` methods, when a non-custom traversal"] # [doc = " is desired for the value of the type of interest passed to that method."] # [doc = " For example, in `MyFolder::try_fold_ty(ty)`, it is valid to call"] # [doc = " `ty.try_super_fold_with(self)`, but any other folding should be done"] # [doc = " with `xyz.try_fold_with(self)`."] fn try_super_fold_with < F : FallibleTypeFolder < I > > (self , folder : & mut F ,) -> Result < Self , F :: Error > ; # [doc = " A convenient alternative to `try_super_fold_with` for use with"] # [doc = " infallible folders. Do not override this method, to ensure coherence"] # [doc = " with `try_super_fold_with`."] fn super_fold_with < F : TypeFolder < I > > (self , folder : & mut F) -> Self ; }
    };
}

TypeSuperFoldable!()