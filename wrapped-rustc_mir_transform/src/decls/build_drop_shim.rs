macro_rules! deps {
    () => {
        DropShimElaborator!();
        MirPatch!();
        Unwind!();
    };
}

macro_rules! build_drop_shim {
    () => {
        deps!();
        fn build_drop_shim < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , ty : Option < Ty < 'tcx > >) -> Body < 'tcx > { debug ! ("build_drop_shim(def_id={:?}, ty={:?})" , def_id , ty) ; assert ! (! matches ! (ty , Some (ty) if ty . is_coroutine ())) ; let args = if let Some (ty) = ty { tcx . mk_args (& [ty . into ()]) } else { GenericArgs :: identity_for_item (tcx , def_id) } ; let sig = tcx . fn_sig (def_id) . instantiate (tcx , args) ; let sig = tcx . instantiate_bound_regions_with_erased (sig) ; let span = tcx . def_span (def_id) ; let source_info = SourceInfo :: outermost (span) ; let return_block = BasicBlock :: new (1) ; let mut blocks = IndexVec :: with_capacity (2) ; let block = | blocks : & mut IndexVec < _ , _ > , kind | { blocks . push (BasicBlockData :: new (Some (Terminator { source_info , kind }) , false)) } ; block (& mut blocks , TerminatorKind :: Goto { target : return_block }) ; block (& mut blocks , TerminatorKind :: Return) ; let source = MirSource :: from_instance (ty :: InstanceKind :: DropGlue (def_id , ty)) ; let mut body = new_body (source , blocks , local_decls_for_sig (& sig , span) , sig . inputs () . len () , span) ; let dropee_ptr = Place :: from (Local :: new (1 + 0)) ; let dropee_ptr = dropee_emit_retag (tcx , & mut body , dropee_ptr , span) ; if ty . is_some () { let patch = { let typing_env = ty :: TypingEnv :: post_analysis (tcx , def_id) ; let mut elaborator = DropShimElaborator { body : & body , patch : MirPatch :: new (& body) , tcx , typing_env , produce_async_drops : false , } ; let dropee = tcx . mk_place_deref (dropee_ptr) ; let resume_block = elaborator . patch . resume_block () ; elaborate_drop (& mut elaborator , source_info , dropee , () , return_block , Unwind :: To (resume_block) , START_BLOCK , None ,) ; elaborator . patch } ; patch . apply (& mut body) ; } body }
    };
}

build_drop_shim!()