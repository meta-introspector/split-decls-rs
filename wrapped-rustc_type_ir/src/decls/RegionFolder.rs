macro_rules! RegionFolder {
    () => {
        # [doc = " Folds over the substructure of a type, visiting its component"] # [doc = " types and all regions that occur *free* within it."] # [doc = ""] # [doc = " That is, function pointer types and trait object can introduce"] # [doc = " new bound regions which are not visited by this visitors as"] # [doc = " they are not free; only regions that occur free will be"] # [doc = " visited by `fld_r`."] pub struct RegionFolder < I , F > { cx : I , # [doc = " Stores the index of a binder *just outside* the stuff we have"] # [doc = " visited. So this begins as INNERMOST; when we pass through a"] # [doc = " binder, it is incremented (via `shift_in`)."] current_index : ty :: DebruijnIndex , # [doc = " Callback invokes for each free region. The `DebruijnIndex`"] # [doc = " points to the binder *just outside* the ones we have passed"] # [doc = " through."] fold_region_fn : F , }
    };
}

RegionFolder!();