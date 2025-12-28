macro_rules! create_query_frame {
    () => {
        pub (crate) fn create_query_frame < 'tcx , K : Copy + DynSend + DynSync + Key + for < 'a > HashStable < StableHashingContext < 'a > > + 'tcx , > (tcx : TyCtxt < 'tcx > , do_describe : fn (TyCtxt < 'tcx > , K) -> String , key : K , kind : DepKind , name : & 'static str ,) -> QueryStackFrame < QueryStackDeferred < 'tcx > > { let def_id = key . key_as_def_id () ; let hash = | | { tcx . with_stable_hashing_context (| mut hcx | { let mut hasher = StableHasher :: new () ; kind . as_usize () . hash_stable (& mut hcx , & mut hasher) ; key . hash_stable (& mut hcx , & mut hasher) ; hasher . finish :: < Hash64 > () }) } ; let def_id_for_ty_in_cycle = key . def_id_for_ty_in_cycle () ; let info = QueryStackDeferred :: new ((tcx , key , kind , name , do_describe) , create_query_frame_extra) ; QueryStackFrame :: new (info , kind , hash , def_id , def_id_for_ty_in_cycle) }
    };
}

create_query_frame!()