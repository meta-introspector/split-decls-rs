macro_rules! deps {
    () => {
        V0SymbolMangler!();
    };
}

macro_rules! mangle {
    () => {
        deps!();
        pub (super) fn mangle < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , instantiating_crate : Option < CrateNum > , is_exportable : bool ,) -> String { let def_id = instance . def_id () ; let args = tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , instance . args) ; let prefix = "_R" ; let mut p : V0SymbolMangler < '_ > = V0SymbolMangler { tcx , start_offset : prefix . len () , is_exportable , paths : FxHashMap :: default () , types : FxHashMap :: default () , consts : FxHashMap :: default () , binders : vec ! [] , out : String :: from (prefix) , } ; let shim_kind = match instance . def { ty :: InstanceKind :: ThreadLocalShim (_) => Some ("tls") , ty :: InstanceKind :: VTableShim (_) => Some ("vtable") , ty :: InstanceKind :: ReifyShim (_ , None) => Some ("reify") , ty :: InstanceKind :: ReifyShim (_ , Some (ReifyReason :: FnPtr)) => Some ("reify_fnptr") , ty :: InstanceKind :: ReifyShim (_ , Some (ReifyReason :: Vtable)) => Some ("reify_vtable") , ty :: InstanceKind :: ConstructCoroutineInClosureShim { receiver_by_ref : true , .. } => { Some ("by_move") } ty :: InstanceKind :: ConstructCoroutineInClosureShim { receiver_by_ref : false , .. } => { Some ("by_ref") } ty :: InstanceKind :: FutureDropPollShim (_ , _ , _) => Some ("drop") , _ => None , } ; if let ty :: InstanceKind :: AsyncDropGlue (_ , ty) = instance . def { let ty :: Coroutine (_ , cor_args) = ty . kind () else { bug ! () ; } ; let drop_ty = cor_args . first () . unwrap () . expect_ty () ; p . print_def_path (def_id , tcx . mk_args (& [GenericArg :: from (drop_ty)])) . unwrap () } else if let Some (shim_kind) = shim_kind { p . path_append_ns (| p | p . print_def_path (def_id , args) , 'S' , 0 , shim_kind) . unwrap () } else { p . print_def_path (def_id , args) . unwrap () } ; if let Some (instantiating_crate) = instantiating_crate { p . print_def_path (instantiating_crate . as_def_id () , & []) . unwrap () ; } std :: mem :: take (& mut p . out) }
    };
}

mangle!();