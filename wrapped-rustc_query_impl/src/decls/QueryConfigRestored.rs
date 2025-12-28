macro_rules! QueryConfigRestored {
    () => {
        # [doc = " This is implemented per query. It allows restoring query values from their erased state"] # [doc = " and constructing a QueryConfig."] trait QueryConfigRestored < 'tcx > { type RestoredValue ; type Config : QueryConfig < QueryCtxt < 'tcx > > ; const NAME : & 'static & 'static str ; fn config (tcx : TyCtxt < 'tcx >) -> Self :: Config ; fn restore (value : < Self :: Config as QueryConfig < QueryCtxt < 'tcx > > > :: Value) -> Self :: RestoredValue ; }
    };
}

QueryConfigRestored!()