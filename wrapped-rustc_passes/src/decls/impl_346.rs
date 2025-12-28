macro_rules! deps {
    () => {
        CaptureCollector!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for CaptureCollector < '_ , 'tcx > { fn visit_path (& mut self , path : & hir :: Path < 'tcx > , _ : HirId) { if let Res :: Local (var_id) = path . res { self . visit_local_use (var_id , path . span) ; } intravisit :: walk_path (self , path) ; } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Closure (closure) = expr . kind && let Some (upvars) = self . tcx . upvars_mentioned (closure . def_id) { for (& var_id , upvar) in upvars { self . visit_local_use (var_id , upvar . span) ; } } intravisit :: walk_expr (self , expr) ; } }
    };
}

impl_346!();