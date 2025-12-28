macro_rules! deps {
    () => {
        PatCtxt!();
    };
}

macro_rules! report_arm_reachability {
    () => {
        deps!();
        # [doc = " Report unreachable arms, if any."] fn report_arm_reachability < 'p , 'tcx > (cx : & PatCtxt < 'p , 'tcx > , report : & UsefulnessReport < 'p , 'tcx > , is_match_arm : bool ,) { let sm = cx . tcx . sess . source_map () ; for (arm , is_useful) in report . arm_usefulness . iter () { if let Usefulness :: Redundant (explanation) = is_useful { let hir_id = arm . arm_data ; let arm_span = cx . tcx . hir_span (hir_id) ; let whole_arm_span = if is_match_arm { let with_whitespace = sm . span_extend_while_whitespace (arm_span) ; if let Some (comma) = sm . span_look_ahead (with_whitespace , "," , Some (1)) { Some (arm_span . to (comma)) } else { Some (arm_span) } } else { None } ; report_unreachable_pattern (cx , hir_id , arm . pat , explanation , whole_arm_span) } } }
    };
}

report_arm_reachability!();