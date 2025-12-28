macro_rules! deps {
    () => {
        Predicate!();
        FallibleTypeFolder!();
        Binder!();
        Const!();
        Ty!();
        TypeFoldable!();
        Region!();
        Interner!();
        Clauses!();
    };
}

macro_rules! TypeFolder {
    () => {
        deps!();
        # [doc = " This trait is implemented for every infallible folding traversal. There is"] # [doc = " a fold method defined for every type of interest. Each such method has a"] # [doc = " default that does an \"identity\" fold. Implementations of these methods"] # [doc = " often fall back to a `super_fold_with` method if the primary argument"] # [doc = " doesn't satisfy a particular condition."] # [doc = ""] # [doc = " A blanket implementation of [`FallibleTypeFolder`] will defer to"] # [doc = " the infallible methods of this trait to ensure that the two APIs"] # [doc = " are coherent."] pub trait TypeFolder < I : Interner > : Sized { fn cx (& self) -> I ; fn fold_binder < T > (& mut self , t : ty :: Binder < I , T >) -> ty :: Binder < I , T > where T : TypeFoldable < I > , { t . super_fold_with (self) } fn fold_ty (& mut self , t : I :: Ty) -> I :: Ty { t . super_fold_with (self) } fn fold_region (& mut self , r : I :: Region) -> I :: Region { r } fn fold_const (& mut self , c : I :: Const) -> I :: Const { c . super_fold_with (self) } fn fold_predicate (& mut self , p : I :: Predicate) -> I :: Predicate { p . super_fold_with (self) } fn fold_clauses (& mut self , c : I :: Clauses) -> I :: Clauses { c . super_fold_with (self) } }
    };
}

TypeFolder!()