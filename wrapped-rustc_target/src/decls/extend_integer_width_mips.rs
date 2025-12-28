macro_rules! deps {
    () => {
        ArgAbi!();
        PassMode!();
        ArgExtension!();
    };
}

macro_rules! extend_integer_width_mips {
    () => {
        deps!();
        fn extend_integer_width_mips < Ty > (arg : & mut ArgAbi < '_ , Ty > , bits : u64) { if let BackendRepr :: Scalar (scalar) = arg . layout . backend_repr && let Primitive :: Int (i , signed) = scalar . primitive () && ! signed && i . size () . bits () == 32 && let PassMode :: Direct (ref mut attrs) = arg . mode { attrs . ext (ArgExtension :: Sext) ; return ; } arg . extend_integer_width_to (bits) ; }
    };
}

extend_integer_width_mips!();