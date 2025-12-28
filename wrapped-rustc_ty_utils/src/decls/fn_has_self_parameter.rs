macro_rules! fn_has_self_parameter {
    () => {
        fn fn_has_self_parameter (tcx : TyCtxt < '_ > , owner_id : hir :: OwnerId) -> bool { matches ! (tcx . fn_arg_idents (owner_id . def_id) , [Some (Ident { name : kw :: SelfLower , .. }) , ..]) }
    };
}

fn_has_self_parameter!();