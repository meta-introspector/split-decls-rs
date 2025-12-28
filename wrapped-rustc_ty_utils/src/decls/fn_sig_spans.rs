macro_rules! fn_sig_spans {
    () => {
        fn fn_sig_spans (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> impl Iterator < Item = Span > { let node = tcx . hir_node_by_def_id (def_id) ; if let Some (decl) = node . fn_decl () { decl . inputs . iter () . map (| ty | ty . span) . chain (iter :: once (decl . output . span ())) } else { bug ! ("unexpected item for fn {def_id:?}: {node:?}") } }
    };
}

fn_sig_spans!();