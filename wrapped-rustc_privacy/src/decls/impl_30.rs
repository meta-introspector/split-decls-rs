macro_rules! deps {
    () => {
        ReportEffectiveVisibility!();
        TestReachabilityVisitor!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'a , 'tcx > TestReachabilityVisitor < 'a , 'tcx > { fn effective_visibility_diagnostic (& self , def_id : LocalDefId) { if self . tcx . has_attr (def_id , sym :: rustc_effective_visibility) { let mut error_msg = String :: new () ; let span = self . tcx . def_span (def_id . to_def_id ()) ; if let Some (effective_vis) = self . effective_visibilities . effective_vis (def_id) { for level in Level :: all_levels () { let vis_str = effective_vis . at_level (level) . to_string (def_id , self . tcx) ; if level != Level :: Direct { error_msg . push_str (", ") ; } error_msg . push_str (& format ! ("{level:?}: {vis_str}")) ; } } else { error_msg . push_str ("not in the table") ; } self . tcx . dcx () . emit_err (ReportEffectiveVisibility { span , descr : error_msg }) ; } } }
    };
}

impl_30!()