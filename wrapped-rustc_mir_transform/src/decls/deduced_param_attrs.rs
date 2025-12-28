macro_rules! deps {
    () => {
        DeduceReadOnly!();
    };
}

macro_rules! deduced_param_attrs {
    () => {
        deps!();
        # [doc = " Returns the deduced parameter attributes for a function."] # [doc = ""] # [doc = " Deduced parameter attributes are those that can only be soundly determined by examining the"] # [doc = " body of the function instead of just the signature. These can be useful for optimization"] # [doc = " purposes on a best-effort basis. We compute them here and store them into the crate metadata so"] # [doc = " dependent crates can use them."] pub (super) fn deduced_param_attrs < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> & 'tcx [DeducedParamAttrs] { if tcx . sess . opts . optimize == OptLevel :: No || tcx . sess . opts . incremental . is_some () { return & [] ; } if tcx . lang_items () . freeze_trait () . is_none () { return & [] ; } let fn_ty = tcx . type_of (def_id) . instantiate_identity () ; if matches ! (fn_ty . kind () , ty :: FnDef (..)) && fn_ty . fn_sig (tcx) . inputs () . skip_binder () . iter () . cloned () . all (type_will_always_be_passed_directly) { return & [] ; } if ! tcx . is_mir_available (def_id) { return & [] ; } let body : & Body < 'tcx > = tcx . optimized_mir (def_id) ; let mut deduce_read_only = DeduceReadOnly :: new (body . arg_count) ; deduce_read_only . visit_body (body) ; let typing_env = body . typing_env (tcx) ; let mut deduced_param_attrs = tcx . arena . alloc_from_iter (body . local_decls . iter () . skip (1) . take (body . arg_count) . enumerate () . map (| (arg_index , local_decl) | DeducedParamAttrs { read_only : ! deduce_read_only . mutable_args . contains (arg_index) && tcx . normalize_erasing_regions (typing_env , local_decl . ty) . is_freeze (tcx , typing_env) , } ,) ,) ; while deduced_param_attrs . last () == Some (& DeducedParamAttrs :: default ()) { let last_index = deduced_param_attrs . len () - 1 ; deduced_param_attrs = & mut deduced_param_attrs [0 .. last_index] ; } deduced_param_attrs }
    };
}

deduced_param_attrs!()