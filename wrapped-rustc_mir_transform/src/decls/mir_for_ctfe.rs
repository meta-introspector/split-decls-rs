macro_rules! mir_for_ctfe {
    () => {
        # [doc = " Compute the MIR that is used during CTFE (and thus has no optimizations run on it)"] fn mir_for_ctfe (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> & Body < '_ > { tcx . arena . alloc (inner_mir_for_ctfe (tcx , def_id)) }
    };
}

mir_for_ctfe!();