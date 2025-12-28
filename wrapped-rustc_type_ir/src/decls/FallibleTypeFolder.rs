macro_rules! deps {
    () => {
        Clauses!();
        Region!();
        Const!();
        TypeFoldable!();
        Interner!();
        TypeFolder!();
        Binder!();
        Predicate!();
        Ty!();
    };
}

macro_rules! FallibleTypeFolder {
    () => {
        deps!();
        # [doc = " This trait is implemented for every folding traversal. There is a fold"] # [doc = " method defined for every type of interest. Each such method has a default"] # [doc = " that does an \"identity\" fold."] # [doc = ""] # [doc = " A blanket implementation of this trait (that defers to the relevant"] # [doc = " method of [`TypeFolder`]) is provided for all infallible folders in"] # [doc = " order to ensure the two APIs are coherent."] pub trait FallibleTypeFolder < I : Interner > : Sized { type Error ; fn cx (& self) -> I ; fn try_fold_binder < T > (& mut self , t : ty :: Binder < I , T >) -> Result < ty :: Binder < I , T > , Self :: Error > where T : TypeFoldable < I > , { t . try_super_fold_with (self) } fn try_fold_ty (& mut self , t : I :: Ty) -> Result < I :: Ty , Self :: Error > { t . try_super_fold_with (self) } fn try_fold_region (& mut self , r : I :: Region) -> Result < I :: Region , Self :: Error > { Ok (r) } fn try_fold_const (& mut self , c : I :: Const) -> Result < I :: Const , Self :: Error > { c . try_super_fold_with (self) } fn try_fold_predicate (& mut self , p : I :: Predicate) -> Result < I :: Predicate , Self :: Error > { p . try_super_fold_with (self) } fn try_fold_clauses (& mut self , c : I :: Clauses) -> Result < I :: Clauses , Self :: Error > { c . try_super_fold_with (self) } }
    };
}

FallibleTypeFolder!();