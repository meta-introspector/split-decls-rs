macro_rules! deps {
    () => {
        EncodeTyOptions!();
        DictKey!();
    };
}

macro_rules! encode_const {
    () => {
        deps!();
        # [doc = " Encodes a const using the Itanium C++ ABI as a literal argument (see"] # [doc = " <https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangling.literal>)."] fn encode_const < 'tcx > (tcx : TyCtxt < 'tcx > , ct : Const < 'tcx > , ct_ty : Ty < 'tcx > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut s = String :: from ('L') ; match ct . kind () { ty :: ConstKind :: Param (..) => { s . push_str (& encode_ty (tcx , ct_ty , dict , options)) ; } ty :: ConstKind :: Value (cv) => { s . push_str (& encode_ty (tcx , cv . ty , dict , options)) ; match cv . ty . kind () { ty :: Int (ity) => { let bits = cv . try_to_bits (tcx , ty :: TypingEnv :: fully_monomorphized ()) . expect ("expected monomorphic const in cfi") ; let val = Integer :: from_int_ty (& tcx , * ity) . size () . sign_extend (bits) as i128 ; if val < 0 { s . push ('n') ; } let _ = write ! (s , "{val}") ; } ty :: Uint (_) => { let val = cv . try_to_bits (tcx , ty :: TypingEnv :: fully_monomorphized ()) . expect ("expected monomorphic const in cfi") ; let _ = write ! (s , "{val}") ; } ty :: Bool => { let val = cv . try_to_bool () . expect ("expected monomorphic const in cfi") ; let _ = write ! (s , "{val}") ; } _ => { bug ! ("encode_const: unexpected type `{:?}`" , cv . ty) ; } } } _ => { bug ! ("encode_const: unexpected kind `{:?}`" , ct . kind ()) ; } } s . push ('E') ; compress (dict , DictKey :: Const (ct) , & mut s) ; s }
    };
}

encode_const!();