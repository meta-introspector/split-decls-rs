macro_rules! deps {
    () => {
        Ty!();
        TypeVisitable!();
        Interner!();
        Predicate!();
        HasEscapingVarsVisitor!();
        Binder!();
        TypeVisitor!();
        Const!();
        FoundEscapingVars!();
        Region!();
        Clauses!();
    };
}

macro_rules! impl_507 {
    () => {
        deps!();
        impl < I : Interner > TypeVisitor < I > for HasEscapingVarsVisitor { type Result = ControlFlow < FoundEscapingVars > ; fn visit_binder < T : TypeVisitable < I > > (& mut self , t : & ty :: Binder < I , T >) -> Self :: Result { self . outer_index . shift_in (1) ; let result = t . super_visit_with (self) ; self . outer_index . shift_out (1) ; result } # [inline] fn visit_ty (& mut self , t : I :: Ty) -> Self :: Result { if t . outer_exclusive_binder () > self . outer_index { ControlFlow :: Break (FoundEscapingVars) } else { ControlFlow :: Continue (()) } } # [inline] fn visit_region (& mut self , r : I :: Region) -> Self :: Result { if r . outer_exclusive_binder () > self . outer_index { ControlFlow :: Break (FoundEscapingVars) } else { ControlFlow :: Continue (()) } } fn visit_const (& mut self , ct : I :: Const) -> Self :: Result { if ct . outer_exclusive_binder () > self . outer_index { ControlFlow :: Break (FoundEscapingVars) } else { ControlFlow :: Continue (()) } } # [inline] fn visit_predicate (& mut self , predicate : I :: Predicate) -> Self :: Result { if predicate . outer_exclusive_binder () > self . outer_index { ControlFlow :: Break (FoundEscapingVars) } else { ControlFlow :: Continue (()) } } # [inline] fn visit_clauses (& mut self , clauses : I :: Clauses) -> Self :: Result { if clauses . outer_exclusive_binder () > self . outer_index { ControlFlow :: Break (FoundEscapingVars) } else { ControlFlow :: Continue (()) } } }
    };
}

impl_507!()