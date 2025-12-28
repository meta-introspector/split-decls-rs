macro_rules! deps {
    () => {
        LayoutConstrainedPlaceVisitor!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'a , 'tcx > for LayoutConstrainedPlaceVisitor < 'a , 'tcx > { fn thir (& self) -> & 'a Thir < 'tcx > { self . thir } fn visit_expr (& mut self , expr : & 'a Expr < 'tcx >) { match expr . kind { ExprKind :: Field { lhs , .. } => { if let ty :: Adt (adt_def , _) = self . thir [lhs] . ty . kind () { if (Bound :: Unbounded , Bound :: Unbounded) != self . tcx . layout_scalar_valid_range (adt_def . did ()) { self . found = true ; } } visit :: walk_expr (self , expr) ; } ExprKind :: Deref { .. } => { } ref kind if ExprCategory :: of (kind) . is_none_or (| cat | cat == ExprCategory :: Place) => { visit :: walk_expr (self , expr) ; } _ => { } } } }
    };
}

impl_176!();