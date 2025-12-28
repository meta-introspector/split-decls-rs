macro_rules! deps {
    () => {
        Liveness!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'tcx > for Liveness < 'a , 'tcx > { fn visit_local (& mut self , local : & 'tcx hir :: LetStmt < 'tcx >) { self . check_unused_vars_in_pat (local . pat , None , None , | spans , hir_id , ln , var | { if local . init . is_some () { self . warn_about_dead_assign (spans , hir_id , ln , var , None) ; } }) ; intravisit :: walk_local (self , local) ; } fn visit_expr (& mut self , ex : & 'tcx Expr < 'tcx >) { check_expr (self , ex) ; intravisit :: walk_expr (self , ex) ; } fn visit_arm (& mut self , arm : & 'tcx hir :: Arm < 'tcx >) { self . check_unused_vars_in_pat (arm . pat , None , None , | _ , _ , _ , _ | { }) ; intravisit :: walk_arm (self , arm) ; } }
    };
}

impl_302!()