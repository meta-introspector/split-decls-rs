macro_rules! closure_saved_names_of_captured_variables {
    () => {
        pub (crate) fn closure_saved_names_of_captured_variables < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> IndexVec < FieldIdx , Symbol > { tcx . closure_captures (def_id) . iter () . map (| captured_place | { let name = captured_place . to_symbol () ; match captured_place . info . capture_kind { ty :: UpvarCapture :: ByValue | ty :: UpvarCapture :: ByUse => name , ty :: UpvarCapture :: ByRef (..) => Symbol :: intern (& format ! ("_ref__{name}")) , } }) . collect () }
    };
}

closure_saved_names_of_captured_variables!()