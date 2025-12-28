macro_rules! deps {
    () => {
        Clauses!();
        Interner!();
        Const!();
        Predicate!();
        Binder!();
        Ty!();
        TypeVisitable!();
        Region!();
    };
}

macro_rules! TypeVisitor {
    () => {
        deps!();
        # [doc = " This trait is implemented for every visiting traversal. There is a visit"] # [doc = " method defined for every type of interest. Each such method has a default"] # [doc = " that recurses into the type's fields in a non-custom fashion."] pub trait TypeVisitor < I : Interner > : Sized { # [cfg (feature = "nightly")] type Result : VisitorResult = () ; # [cfg (not (feature = "nightly"))] type Result : VisitorResult ; fn visit_binder < T : TypeVisitable < I > > (& mut self , t : & ty :: Binder < I , T >) -> Self :: Result { t . super_visit_with (self) } fn visit_ty (& mut self , t : I :: Ty) -> Self :: Result { t . super_visit_with (self) } fn visit_region (& mut self , r : I :: Region) -> Self :: Result { if let ty :: ReError (guar) = r . kind () { self . visit_error (guar) } else { Self :: Result :: output () } } fn visit_const (& mut self , c : I :: Const) -> Self :: Result { c . super_visit_with (self) } fn visit_predicate (& mut self , p : I :: Predicate) -> Self :: Result { p . super_visit_with (self) } fn visit_clauses (& mut self , c : I :: Clauses) -> Self :: Result { c . super_visit_with (self) } fn visit_error (& mut self , _guar : I :: ErrorGuaranteed) -> Self :: Result { Self :: Result :: output () } }
    };
}

TypeVisitor!();