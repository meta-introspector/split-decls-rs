macro_rules! build_thread_local_shim {
    () => {
        fn build_thread_local_shim < 'tcx > (tcx : TyCtxt < 'tcx > , instance : ty :: InstanceKind < 'tcx > ,) -> Body < 'tcx > { let def_id = instance . def_id () ; let span = tcx . def_span (def_id) ; let source_info = SourceInfo :: outermost (span) ; let blocks = IndexVec :: from_raw (vec ! [BasicBlockData :: new_stmts (vec ! [Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: return_place () , Rvalue :: ThreadLocalRef (def_id) ,))) ,)] , Some (Terminator { source_info , kind : TerminatorKind :: Return }) , false ,)]) ; new_body (MirSource :: from_instance (instance) , blocks , IndexVec :: from_raw (vec ! [LocalDecl :: new (tcx . thread_local_ptr_ty (def_id) , span)]) , 0 , span ,) }
    };
}

build_thread_local_shim!();