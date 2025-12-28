macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: AggregateKind < 'tcx > { type T = crate :: mir :: AggregateKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { mir :: AggregateKind :: Array (ty) => { crate :: mir :: AggregateKind :: Array (ty . stable (tables , cx)) } mir :: AggregateKind :: Tuple => crate :: mir :: AggregateKind :: Tuple , mir :: AggregateKind :: Adt (def_id , var_idx , generic_arg , user_ty_index , field_idx) => { crate :: mir :: AggregateKind :: Adt (tables . adt_def (* def_id) , var_idx . stable (tables , cx) , generic_arg . stable (tables , cx) , user_ty_index . map (| idx | idx . index ()) , field_idx . map (| idx | idx . index ()) ,) } mir :: AggregateKind :: Closure (def_id , generic_arg) => crate :: mir :: AggregateKind :: Closure (tables . closure_def (* def_id) , generic_arg . stable (tables , cx) ,) , mir :: AggregateKind :: Coroutine (def_id , generic_arg) => { crate :: mir :: AggregateKind :: Coroutine (tables . coroutine_def (* def_id) , generic_arg . stable (tables , cx) ,) } mir :: AggregateKind :: CoroutineClosure (def_id , generic_args) => { crate :: mir :: AggregateKind :: CoroutineClosure (tables . coroutine_closure_def (* def_id) , generic_args . stable (tables , cx) ,) } mir :: AggregateKind :: RawPtr (ty , mutability) => crate :: mir :: AggregateKind :: RawPtr (ty . stable (tables , cx) , mutability . stable (tables , cx) ,) , } } }
    };
}

impl_151!();