macro_rules! deps {
    () => {
        ImplsOrderVisitor!();
    };
}

macro_rules! stable_order_of_exportable_impls {
    () => {
        deps!();
        # [doc = " During symbol mangling rustc uses a special index to distinguish between two impls of"] # [doc = " the same type in the same module(See `DisambiguatedDefPathData`). For exportable items"] # [doc = " we cannot use the current approach because it is dependent on the compiler's"] # [doc = " implementation."] # [doc = ""] # [doc = " In order to make disambiguation independent of the compiler version we can assign an"] # [doc = " id to each impl according to the relative order of elements in the source code."] fn stable_order_of_exportable_impls < 'tcx > (tcx : TyCtxt < 'tcx > , _ : LocalCrate ,) -> & 'tcx FxIndexMap < DefId , usize > { if ! tcx . crate_types () . contains (& CrateType :: Sdylib) && ! tcx . is_sdylib_interface_build () { return tcx . arena . alloc (FxIndexMap :: < DefId , usize > :: default ()) ; } let mut vis = ImplsOrderVisitor :: new (tcx) ; tcx . hir_walk_toplevel_module (& mut vis) ; tcx . arena . alloc (vis . order) }
    };
}

stable_order_of_exportable_impls!()