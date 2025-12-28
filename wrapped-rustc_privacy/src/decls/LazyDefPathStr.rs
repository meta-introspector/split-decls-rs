macro_rules! LazyDefPathStr {
    () => {
        # [doc = " Generic infrastructure used to implement specific visitors below."] struct LazyDefPathStr < 'tcx > { def_id : DefId , tcx : TyCtxt < 'tcx > , }
    };
}

LazyDefPathStr!()