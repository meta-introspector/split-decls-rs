macro_rules! deps {
    () => {
        MonoItems!();
    };
}

macro_rules! visit_fn_use {
    () => {
        deps!();
        # [doc = " For every call of this function in the visitor, make sure there is a matching call in the"] # [doc = " `mentioned_items` pass!"] fn visit_fn_use < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , is_direct_call : bool , source : Span , output : & mut MonoItems < 'tcx > ,) { if let ty :: FnDef (def_id , args) = * ty . kind () { let instance = if is_direct_call { ty :: Instance :: expect_resolve (tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , args , source ,) } else { match ty :: Instance :: resolve_for_fn_ptr (tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , args ,) { Some (instance) => instance , _ => bug ! ("failed to resolve instance for {ty}") , } } ; visit_instance_use (tcx , instance , is_direct_call , source , output) ; } }
    };
}

visit_fn_use!();