macro_rules! deps {
    () => {
        EncodeTyOptions!();
        DictKey!();
    };
}

macro_rules! encode_args {
    () => {
        deps!();
        # [doc = " Encodes args using the Itanium C++ ABI with vendor extended type qualifiers and types for Rust"] # [doc = " types that are not used at the FFI boundary."] fn encode_args < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > , for_def : DefId , has_erased_self : bool , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut s = String :: new () ; let args : Vec < GenericArg < '_ > > = args . iter () . collect () ; if ! args . is_empty () { s . push ('I') ; let def_generics = tcx . generics_of (for_def) ; for (n , arg) in args . iter () . enumerate () { match arg . kind () { GenericArgKind :: Lifetime (region) => { s . push_str (& encode_region (region , dict)) ; } GenericArgKind :: Type (ty) => { s . push_str (& encode_ty (tcx , ty , dict , options)) ; } GenericArgKind :: Const (c) => { let n = n + (has_erased_self as usize) ; let ct_ty = tcx . type_of (def_generics . param_at (n , tcx) . def_id) . instantiate_identity () ; s . push_str (& encode_const (tcx , c , ct_ty , dict , options)) ; } } } s . push ('E') ; } s }
    };
}

encode_args!()