macro_rules! deps {
    () => {
        CollectLitsVisitor!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for CollectLitsVisitor < 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) { if let hir :: ExprKind :: Lit (_) = expr . kind { self . lit_exprs . push (expr) ; } intravisit :: walk_expr (self , expr) ; } }
    };
}

impl_293!()