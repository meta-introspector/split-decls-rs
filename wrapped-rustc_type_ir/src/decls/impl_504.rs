macro_rules! deps {
    () => {
        Clauses!();
        Predicate!();
        Binder!();
        Ty!();
        Interner!();
        TypeVisitor!();
        Const!();
        HasTypeFlagsVisitor!();
        Region!();
        TypeVisitable!();
        FoundFlags!();
    };
}

macro_rules! impl_504 {
    () => {
        deps!();
        impl < I : Interner > TypeVisitor < I > for HasTypeFlagsVisitor { type Result = ControlFlow < FoundFlags > ; fn visit_binder < T : TypeVisitable < I > > (& mut self , t : & ty :: Binder < I , T >) -> Self :: Result { if self . flags . intersects (TypeFlags :: HAS_BINDER_VARS) && ! t . bound_vars () . is_empty () { return ControlFlow :: Break (FoundFlags) ; } t . super_visit_with (self) } # [inline] fn visit_ty (& mut self , t : I :: Ty) -> Self :: Result { let flags = t . flags () ; if flags . intersects (self . flags) { ControlFlow :: Break (FoundFlags) } else { ControlFlow :: Continue (()) } } # [inline] fn visit_region (& mut self , r : I :: Region) -> Self :: Result { let flags = r . flags () ; if flags . intersects (self . flags) { ControlFlow :: Break (FoundFlags) } else { ControlFlow :: Continue (()) } } # [inline] fn visit_const (& mut self , c : I :: Const) -> Self :: Result { if c . flags () . intersects (self . flags) { ControlFlow :: Break (FoundFlags) } else { ControlFlow :: Continue (()) } } # [inline] fn visit_predicate (& mut self , predicate : I :: Predicate) -> Self :: Result { if predicate . flags () . intersects (self . flags) { ControlFlow :: Break (FoundFlags) } else { ControlFlow :: Continue (()) } } # [inline] fn visit_clauses (& mut self , clauses : I :: Clauses) -> Self :: Result { if clauses . flags () . intersects (self . flags) { ControlFlow :: Break (FoundFlags) } else { ControlFlow :: Continue (()) } } # [inline] fn visit_error (& mut self , _guar : < I as Interner > :: ErrorGuaranteed) -> Self :: Result { if self . flags . intersects (TypeFlags :: HAS_ERROR) { ControlFlow :: Break (FoundFlags) } else { ControlFlow :: Continue (()) } } }
    };
}

impl_504!();