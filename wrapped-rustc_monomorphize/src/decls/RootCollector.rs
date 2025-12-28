macro_rules! deps {
    () => {
        MonoItemCollectionStrategy!();
        MonoItems!();
    };
}

macro_rules! RootCollector {
    () => {
        deps!();
        struct RootCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , strategy : MonoItemCollectionStrategy , output : & 'a mut MonoItems < 'tcx > , entry_fn : Option < (DefId , EntryFnType) > , }
    };
}

RootCollector!();