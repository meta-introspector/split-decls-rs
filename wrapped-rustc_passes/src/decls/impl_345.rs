macro_rules! deps {
    () => {
        CaptureCollector!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl CaptureCollector < '_ , '_ > { fn visit_local_use (& mut self , var_id : HirId , span : Span) { if ! self . locals . contains (& var_id) { self . upvars . entry (var_id) . or_insert (hir :: Upvar { span }) ; } } }
    };
}

impl_345!();