macro_rules! deps {
    () => {
        TailCallCkVisitor!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'a , 'tcx > for TailCallCkVisitor < 'a , 'tcx > { fn thir (& self) -> & 'a Thir < 'tcx > { & self . thir } fn visit_expr (& mut self , expr : & 'a Expr < 'tcx >) { ensure_sufficient_stack (| | { if let ExprKind :: Become { value } = expr . kind { let call = & self . thir [value] ; self . check_tail_call (call , expr) ; } visit :: walk_expr (self , expr) ; }) ; } }
    };
}

impl_169!()