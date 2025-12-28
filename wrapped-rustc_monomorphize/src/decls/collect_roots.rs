macro_rules! deps {
    () => {
        MonoItemCollectionStrategy!();
        MonoItems!();
        RootCollector!();
    };
}

macro_rules! collect_roots {
    () => {
        deps!();
        # [instrument (skip (tcx , mode) , level = "debug")] fn collect_roots (tcx : TyCtxt < '_ > , mode : MonoItemCollectionStrategy) -> Vec < MonoItem < '_ > > { debug ! ("collecting roots") ; let mut roots = MonoItems :: new () ; { let entry_fn = tcx . entry_fn (()) ; debug ! ("collect_roots: entry_fn = {:?}" , entry_fn) ; let mut collector = RootCollector { tcx , strategy : mode , entry_fn , output : & mut roots } ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . free_items () { collector . process_item (id) ; } for id in crate_items . impl_items () { collector . process_impl_item (id) ; } for id in crate_items . nested_bodies () { collector . process_nested_body (id) ; } collector . push_extra_entry_roots () ; } roots . into_iter () . filter_map (| Spanned { node : mono_item , .. } | { mono_item . is_instantiable (tcx) . then_some (mono_item) }) . collect () }
    };
}

collect_roots!()