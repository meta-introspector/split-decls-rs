macro_rules! deps {
    () => {
        MonoItems!();
    };
}

macro_rules! collect_alloc {
    () => {
        deps!();
        # [doc = " Scans the CTFE alloc in order to find function pointers and statics that must be monomorphized."] fn collect_alloc < 'tcx > (tcx : TyCtxt < 'tcx > , alloc_id : AllocId , output : & mut MonoItems < 'tcx >) { match tcx . global_alloc (alloc_id) { GlobalAlloc :: Static (def_id) => { assert ! (! tcx . is_thread_local_static (def_id)) ; let instance = Instance :: mono (tcx , def_id) ; if tcx . should_codegen_locally (instance) { trace ! ("collecting static {:?}" , def_id) ; output . push (dummy_spanned (MonoItem :: Static (def_id))) ; } } GlobalAlloc :: Memory (alloc) => { trace ! ("collecting {:?} with {:#?}" , alloc_id , alloc) ; let ptrs = alloc . inner () . provenance () . ptrs () ; if ! ptrs . is_empty () { rustc_data_structures :: stack :: ensure_sufficient_stack (move | | { for & prov in ptrs . values () { collect_alloc (tcx , prov . alloc_id () , output) ; } }) ; } } GlobalAlloc :: Function { instance , .. } => { if tcx . should_codegen_locally (instance) { trace ! ("collecting {:?} with {:#?}" , alloc_id , instance) ; output . push (create_fn_mono_item (tcx , instance , DUMMY_SP)) ; } } GlobalAlloc :: VTable (ty , dyn_ty) => { let alloc_id = tcx . vtable_allocation ((ty , dyn_ty . principal () . map (| principal | tcx . instantiate_bound_regions_with_erased (principal)) ,)) ; collect_alloc (tcx , alloc_id , output) } GlobalAlloc :: TypeId { .. } => { } } }
    };
}

collect_alloc!()