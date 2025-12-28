macro_rules! items_of_instance {
    () => {
        fn items_of_instance < 'tcx > (tcx : TyCtxt < 'tcx > , (instance , mode) : (Instance < 'tcx > , CollectionMode) ,) -> (& 'tcx [Spanned < MonoItem < 'tcx > >] , & 'tcx [Spanned < MonoItem < 'tcx > >]) { let (used_items , mentioned_items) = collect_items_of_instance (tcx , instance , mode) ; let used_items = tcx . arena . alloc_from_iter (used_items) ; let mentioned_items = tcx . arena . alloc_from_iter (mentioned_items) ; (used_items , mentioned_items) }
    };
}

items_of_instance!();