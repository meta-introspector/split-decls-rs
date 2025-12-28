macro_rules! deps {
    () => {
        Target!();
        ArgAbi!();
    };
}

macro_rules! softfloat_float_abi {
    () => {
        deps!();
        fn softfloat_float_abi < Ty > (target : & Target , arg : & mut ArgAbi < '_ , Ty >) { if target . abi != "softfloat" { return ; } if let BackendRepr :: Scalar (s) = arg . layout . backend_repr && let Primitive :: Float (f) = s . primitive () { arg . cast_to (Reg { kind : RegKind :: Integer , size : f . size () }) ; } else if let BackendRepr :: ScalarPair (s1 , s2) = arg . layout . backend_repr && (matches ! (s1 . primitive () , Primitive :: Float (_)) || matches ! (s2 . primitive () , Primitive :: Float (_))) { if arg . layout . size . bits () <= target . pointer_width . into () { arg . cast_to (Reg { kind : RegKind :: Integer , size : arg . layout . size }) ; } else { arg . make_indirect () ; } } }
    };
}

softfloat_float_abi!()