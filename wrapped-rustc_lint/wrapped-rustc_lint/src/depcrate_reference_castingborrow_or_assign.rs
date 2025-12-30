// Generated macro for borrow_or_assign (function)
macro_rules! Depcrate_reference_castingborrow_or_assign {
() => {
// Module: crate::reference_casting
// Provides: {"borrow_or_assign"}
// Dependencies: {}
fn borrow_or_assign < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < 'tcx > ,) -> Option < (& 'tcx Expr < 'tcx > , PatternKind) > { fn deref_assign_or_addr_of < 'tcx > (expr : & 'tcx Expr < 'tcx > ,) -> Option < (& 'tcx Expr < 'tcx > , PatternKind) > { let (inner , pat) = if let ExprKind :: AddrOf (_ , mutbl , expr) = expr . kind { (expr , PatternKind :: Borrow { mutbl }) } else if let ExprKind :: Assign (expr , _ , _) = expr . kind { (expr , PatternKind :: Assign) } else if let ExprKind :: AssignOp (_ , expr , _) = expr . kind { (expr , PatternKind :: Assign) } else { return None ; } ; let ExprKind :: Unary (UnOp :: Deref , e) = & inner . kind else { return None ; } ; Some ((e , pat)) } fn ptr_write < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < 'tcx > ,) -> Option < (& 'tcx Expr < 'tcx > , PatternKind) > { if let ExprKind :: Call (path , [arg_ptr , _arg_val]) = e . kind && let ExprKind :: Path (ref qpath) = path . kind && let Some (def_id) = cx . qpath_res (qpath , path . hir_id) . opt_def_id () && matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: ptr_write | sym :: ptr_write_volatile | sym :: ptr_write_unaligned)) { Some ((arg_ptr , PatternKind :: Assign)) } else { None } } deref_assign_or_addr_of (e) . or_else (| | ptr_write (cx , e)) }
};
}
