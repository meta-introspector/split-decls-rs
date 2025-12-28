macro_rules! deps {
    () => {
        TransformTyOptions!();
        EncodeTyOptions!();
        DictKey!();
        TransformTy!();
    };
}

macro_rules! encode_fnsig {
    () => {
        deps!();
        # [doc = " Encodes a FnSig using the Itanium C++ ABI with vendor extended type qualifiers and types for"] # [doc = " Rust types that are not used at the FFI boundary."] fn encode_fnsig < 'tcx > (tcx : TyCtxt < 'tcx > , fn_sig : & FnSig < 'tcx > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : TypeIdOptions ,) -> String { let mut s = String :: from ("F") ; let mut encode_ty_options = EncodeTyOptions :: from_bits (options . bits ()) . unwrap_or_else (| | bug ! ("encode_fnsig: invalid option(s) `{:?}`" , options . bits ())) ; match fn_sig . abi { ExternAbi :: C { .. } => { encode_ty_options . insert (EncodeTyOptions :: GENERALIZE_REPR_C) ; } _ => { encode_ty_options . remove (EncodeTyOptions :: GENERALIZE_REPR_C) ; } } let transform_ty_options = TransformTyOptions :: from_bits (options . bits ()) . unwrap_or_else (| | bug ! ("encode_fnsig: invalid option(s) `{:?}`" , options . bits ())) ; let mut type_folder = TransformTy :: new (tcx , transform_ty_options) ; let ty = fn_sig . output () . fold_with (& mut type_folder) ; s . push_str (& encode_ty (tcx , ty , dict , encode_ty_options)) ; let tys = fn_sig . inputs () ; if ! tys . is_empty () { for ty in tys { let ty = ty . fold_with (& mut type_folder) ; s . push_str (& encode_ty (tcx , ty , dict , encode_ty_options)) ; } if fn_sig . c_variadic { s . push ('z') ; } } else if fn_sig . c_variadic { s . push ('z') ; } else { s . push ('v') } s . push ('E') ; s }
    };
}

encode_fnsig!();