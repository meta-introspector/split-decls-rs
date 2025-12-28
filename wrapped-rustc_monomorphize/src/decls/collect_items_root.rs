macro_rules! deps {
    () => {
        SharedState!();
    };
}

macro_rules! collect_items_root {
    () => {
        deps!();
        fn collect_items_root < 'tcx > (tcx : TyCtxt < 'tcx > , starting_item : Spanned < MonoItem < 'tcx > > , state : & SharedState < 'tcx > , recursion_limit : Limit ,) { if ! state . visited . lock_mut () . insert (starting_item . node) { return ; } let mut recursion_depths = DefIdMap :: default () ; collect_items_rec (tcx , starting_item , state , & mut recursion_depths , recursion_limit , CollectionMode :: UsedItems ,) ; }
    };
}

collect_items_root!();