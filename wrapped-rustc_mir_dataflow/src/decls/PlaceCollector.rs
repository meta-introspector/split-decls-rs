macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! PlaceCollector {
    () => {
        deps!();
        struct PlaceCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , map : & 'a mut Map < 'tcx > , assignments : FxIndexSet < (PlaceIndex , PlaceIndex) > , }
    };
}

PlaceCollector!()