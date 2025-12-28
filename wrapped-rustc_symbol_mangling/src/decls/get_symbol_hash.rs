macro_rules! get_symbol_hash {
    () => {
        fn get_symbol_hash < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , item_type : Ty < 'tcx > , instantiating_crate : Option < CrateNum > ,) -> Hash64 { let def_id = instance . def_id () ; let args = instance . args ; debug ! ("get_symbol_hash(def_id={:?}, parameters={:?})" , def_id , args) ; tcx . with_stable_hashing_context (| mut hcx | { let mut hasher = StableHasher :: new () ; tcx . def_path_hash (def_id) . hash_stable (& mut hcx , & mut hasher) ; assert ! (! item_type . has_erasable_regions ()) ; hcx . while_hashing_spans (false , | hcx | { item_type . hash_stable (hcx , & mut hasher) ; if let ty :: FnDef (..) = item_type . kind () { item_type . fn_sig (tcx) . hash_stable (hcx , & mut hasher) ; } args . hash_stable (hcx , & mut hasher) ; if let Some (instantiating_crate) = instantiating_crate { tcx . def_path_hash (instantiating_crate . as_def_id ()) . stable_crate_id () . hash_stable (hcx , & mut hasher) ; } discriminant (& instance . def) . hash_stable (hcx , & mut hasher) ; }) ; hasher . finish :: < Hash64 > () }) }
    };
}

get_symbol_hash!();