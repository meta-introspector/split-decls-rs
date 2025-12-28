macro_rules! deps {
    () => {
        IsThirPolymorphic!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < 'a , 'tcx > visit :: Visitor < 'a , 'tcx > for IsThirPolymorphic < 'a , 'tcx > { fn thir (& self) -> & 'a thir :: Thir < 'tcx > { self . thir } # [instrument (skip (self) , level = "debug")] fn visit_expr (& mut self , expr : & 'a thir :: Expr < 'tcx >) { self . is_poly |= self . expr_is_poly (expr) ; if ! self . is_poly { visit :: walk_expr (self , expr) } } # [instrument (skip (self) , level = "debug")] fn visit_pat (& mut self , pat : & 'a thir :: Pat < 'tcx >) { self . is_poly |= self . pat_is_poly (pat) ; if ! self . is_poly { visit :: walk_pat (self , pat) ; } } }
    };
}

impl_41!()