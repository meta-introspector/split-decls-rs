macro_rules! deps {
    () => {
        MonoItems!();
    };
}

macro_rules! collect_const_value {
    () => {
        deps!();
        # [instrument (skip (tcx , output) , level = "debug")] fn collect_const_value < 'tcx > (tcx : TyCtxt < 'tcx > , value : mir :: ConstValue , output : & mut MonoItems < 'tcx > ,) { match value { mir :: ConstValue :: Scalar (Scalar :: Ptr (ptr , _size)) => { collect_alloc (tcx , ptr . provenance . alloc_id () , output) } mir :: ConstValue :: Indirect { alloc_id , .. } | mir :: ConstValue :: Slice { alloc_id , meta : _ } => collect_alloc (tcx , alloc_id , output) , _ => { } } }
    };
}

collect_const_value!()