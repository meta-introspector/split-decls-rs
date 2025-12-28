macro_rules! Lint {
    () => {
        struct Lint < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , when : String , body : & 'a Body < 'tcx > , is_fn_like : bool , always_live_locals : & 'a DenseBitSet < Local > , maybe_storage_live : ResultsCursor < 'a , 'tcx , MaybeStorageLive < 'a > > , maybe_storage_dead : ResultsCursor < 'a , 'tcx , MaybeStorageDead < 'a > > , places : FxHashSet < PlaceRef < 'tcx > > , }
    };
}

Lint!();