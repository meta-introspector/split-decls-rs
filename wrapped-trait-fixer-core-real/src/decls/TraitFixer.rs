macro_rules! TraitFixer {
    () => {
        pub struct TraitFixer < 'tcx , C > where C : ConfigTrait , { pub tcx : TyCtxt < 'tcx > , pub config : C , pub fixes : Vec < Fix < rustc_span :: Span , hir :: def_id :: DefId , hir :: ItemId > > , }
    };
}

TraitFixer!();