macro_rules! PrivateItemsInPublicInterfacesChecker {
    () => {
        struct PrivateItemsInPublicInterfacesChecker < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , effective_visibilities : & 'a EffectiveVisibilities , }
    };
}

PrivateItemsInPublicInterfacesChecker!();