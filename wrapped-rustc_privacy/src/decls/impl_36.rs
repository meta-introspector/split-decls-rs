macro_rules! deps {
    () => {
        TypePrivacyVisitor!();
        ItemIsPrivate!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'tcx > TypePrivacyVisitor < 'tcx > { fn item_is_accessible (& self , did : DefId) -> bool { self . tcx . visibility (did) . is_accessible_from (self . module_def_id , self . tcx) } fn check_expr_pat_type (& mut self , id : hir :: HirId , span : Span) -> bool { self . span = span ; let typeck_results = self . maybe_typeck_results . unwrap_or_else (| | span_bug ! (span , "`hir::Expr` or `hir::Pat` outside of a body")) ; let result : ControlFlow < () > = try { self . visit (typeck_results . node_type (id)) ? ; self . visit (typeck_results . node_args (id)) ? ; if let Some (adjustments) = typeck_results . adjustments () . get (id) { adjustments . iter () . try_for_each (| adjustment | self . visit (adjustment . target)) ? ; } } ; result . is_break () } fn check_def_id (& mut self , def_id : DefId , kind : & str , descr : & dyn fmt :: Display) -> bool { let is_error = ! self . item_is_accessible (def_id) ; if is_error { self . tcx . dcx () . emit_err (ItemIsPrivate { span : self . span , kind , descr : descr . into () }) ; } is_error } }
    };
}

impl_36!();