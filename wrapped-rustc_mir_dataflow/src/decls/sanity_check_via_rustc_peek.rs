macro_rules! deps {
    () => {
        PeekMustBePlaceOrRefPlace!();
        RustcPeekAt!();
        ResultsCursor!();
        PeekCall!();
        PeekCallKind!();
    };
}

macro_rules! sanity_check_via_rustc_peek {
    () => {
        deps!();
        # [doc = " This function scans `mir` for all calls to the intrinsic"] # [doc = " `rustc_peek` that have the expression form `rustc_peek(&expr)`."] # [doc = ""] # [doc = " For each such call, determines what the dataflow bit-state is for"] # [doc = " the L-value corresponding to `expr`; if the bit-state is a 1, then"] # [doc = " that call to `rustc_peek` is ignored by the sanity check. If the"] # [doc = " bit-state is a 0, then this pass emits an error message saying"] # [doc = " \"rustc_peek: bit not set\"."] # [doc = ""] # [doc = " The intention is that one can write unit tests for dataflow by"] # [doc = " putting code into a UI test and using `rustc_peek` to"] # [doc = " make observations about the results of dataflow static analyses."] # [doc = ""] # [doc = " (If there are any calls to `rustc_peek` that do not match the"] # [doc = " expression form above, then that emits an error as well, but those"] # [doc = " errors are not intended to be used for unit tests.)"] fn sanity_check_via_rustc_peek < 'tcx , A > (tcx : TyCtxt < 'tcx > , mut cursor : ResultsCursor < '_ , 'tcx , A >) where A : RustcPeekAt < 'tcx > , { let def_id = cursor . body () . source . def_id () ; debug ! ("sanity_check_via_rustc_peek def_id: {:?}" , def_id) ; let peek_calls = cursor . body () . basic_blocks . iter_enumerated () . filter_map (| (bb , block_data) | { PeekCall :: from_terminator (tcx , block_data . terminator ()) . map (| call | (bb , block_data , call)) }) ; for (bb , block_data , call) in peek_calls { let (statement_index , peek_rval) = block_data . statements . iter () . enumerate () . find_map (| (i , stmt) | value_assigned_to_local (stmt , call . arg) . map (| rval | (i , rval))) . expect ("call to rustc_peek should be preceded by \
                    assignment to temporary holding its argument" ,) ; match (call . kind , peek_rval) { (PeekCallKind :: ByRef , mir :: Rvalue :: Ref (_ , _ , place)) | (PeekCallKind :: ByVal , mir :: Rvalue :: Use (mir :: Operand :: Move (place) | mir :: Operand :: Copy (place)) ,) => { let loc = Location { block : bb , statement_index } ; cursor . seek_before_primary_effect (loc) ; let state = cursor . get () ; let analysis = cursor . analysis () ; analysis . peek_at (tcx , * place , state , call) ; } _ => { tcx . dcx () . emit_err (PeekMustBePlaceOrRefPlace { span : call . span }) ; } } } }
    };
}

sanity_check_via_rustc_peek!()