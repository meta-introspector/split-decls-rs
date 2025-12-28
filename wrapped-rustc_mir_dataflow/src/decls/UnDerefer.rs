macro_rules! UnDerefer {
    () => {
        # [doc = " Used for reverting changes made by `DerefSeparator`"] # [derive (Default , Debug)] pub (crate) struct UnDerefer < 'tcx > { deref_chains : FxHashMap < Local , Vec < PlaceRef < 'tcx > > > , }
    };
}

UnDerefer!()