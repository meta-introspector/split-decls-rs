macro_rules! deps {
    () => {
        EncodeTyOptions!();
        DictKey!();
    };
}

macro_rules! encode_predicate {
    () => {
        deps!();
        # [doc = " Encodes a predicate using the Itanium C++ ABI with vendor extended type qualifiers and types for"] # [doc = " Rust types that are not used at the FFI boundary."] fn encode_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , predicate : ty :: PolyExistentialPredicate < 'tcx > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut s = String :: new () ; match predicate . as_ref () . skip_binder () { ty :: ExistentialPredicate :: Trait (trait_ref) => { let name = encode_ty_name (tcx , trait_ref . def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; s . push_str (& encode_args (tcx , trait_ref . args , trait_ref . def_id , true , dict , options)) ; } ty :: ExistentialPredicate :: Projection (projection) => { let name = encode_ty_name (tcx , projection . def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; s . push_str (& encode_args (tcx , projection . args , projection . def_id , true , dict , options)) ; match projection . term . kind () { TermKind :: Ty (ty) => s . push_str (& encode_ty (tcx , ty , dict , options)) , TermKind :: Const (c) => s . push_str (& encode_const (tcx , c , tcx . type_of (projection . def_id) . instantiate (tcx , projection . args) , dict , options ,)) , } } ty :: ExistentialPredicate :: AutoTrait (def_id) => { let name = encode_ty_name (tcx , * def_id) ; let _ = write ! (s , "u{}{}" , name . len () , name) ; } } ; compress (dict , DictKey :: Predicate (* predicate . as_ref () . skip_binder ()) , & mut s) ; s }
    };
}

encode_predicate!();