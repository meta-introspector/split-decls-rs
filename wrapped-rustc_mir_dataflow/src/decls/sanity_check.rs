macro_rules! deps {
    () => {
        MaybeInitializedPlaces!();
        StopAfterDataFlowEndedCompilation!();
        MaybeLiveLocals!();
        MoveData!();
        MaybeUninitializedPlaces!();
    };
}

macro_rules! sanity_check {
    () => {
        deps!();
        pub fn sanity_check < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { let def_id = body . source . def_id () ; if ! tcx . has_attr (def_id , sym :: rustc_mir) { debug ! ("skipping rustc_peek::SanityCheck on {}" , tcx . def_path_str (def_id)) ; return ; } else { debug ! ("running rustc_peek::SanityCheck on {}" , tcx . def_path_str (def_id)) ; } let move_data = MoveData :: gather_moves (body , tcx , | _ | true) ; if has_rustc_mir_with (tcx , def_id , sym :: rustc_peek_maybe_init) . is_some () { let flow_inits = MaybeInitializedPlaces :: new (tcx , body , & move_data) . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; sanity_check_via_rustc_peek (tcx , flow_inits) ; } if has_rustc_mir_with (tcx , def_id , sym :: rustc_peek_maybe_uninit) . is_some () { let flow_uninits = MaybeUninitializedPlaces :: new (tcx , body , & move_data) . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; sanity_check_via_rustc_peek (tcx , flow_uninits) ; } if has_rustc_mir_with (tcx , def_id , sym :: rustc_peek_liveness) . is_some () { let flow_liveness = MaybeLiveLocals . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; sanity_check_via_rustc_peek (tcx , flow_liveness) ; } if has_rustc_mir_with (tcx , def_id , sym :: stop_after_dataflow) . is_some () { tcx . dcx () . emit_fatal (StopAfterDataFlowEndedCompilation) ; } }
    };
}

sanity_check!();