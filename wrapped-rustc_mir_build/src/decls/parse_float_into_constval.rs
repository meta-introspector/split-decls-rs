macro_rules! parse_float_into_constval {
    () => {
        fn parse_float_into_constval (num : Symbol , float_ty : ty :: FloatTy , neg : bool) -> Option < ConstValue > { parse_float_into_scalar (num , float_ty , neg) . map (| s | ConstValue :: Scalar (s . into ())) }
    };
}

parse_float_into_constval!();