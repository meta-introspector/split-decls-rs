macro_rules! asyncness {
    () => {
        # [doc = " Check if a function is async."] fn asyncness (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> ty :: Asyncness { let node = tcx . hir_node_by_def_id (def_id) ; node . fn_sig () . map_or (ty :: Asyncness :: No , | sig | match sig . header . asyncness { hir :: IsAsync :: Async (_) => ty :: Asyncness :: Yes , hir :: IsAsync :: NotAsync => ty :: Asyncness :: No , }) }
    };
}

asyncness!()