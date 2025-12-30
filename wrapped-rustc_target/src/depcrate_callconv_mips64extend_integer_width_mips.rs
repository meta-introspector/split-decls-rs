// Generated macro for extend_integer_width_mips (function)
macro_rules! Depcrate_callconv_mips64extend_integer_width_mips {
() => {
// Module: crate::callconv::mips64
// Provides: {"extend_integer_width_mips"}
// Dependencies: {}
fn extend_integer_width_mips < Ty > (arg : & mut ArgAbi < '_ , Ty > , bits : u64) { if let BackendRepr :: Scalar (scalar) = arg . layout . backend_repr && let Primitive :: Int (i , signed) = scalar . primitive () && ! signed && i . size () . bits () == 32 && let PassMode :: Direct (ref mut attrs) = arg . mode { attrs . ext (ArgExtension :: Sext) ; return ; } arg . extend_integer_width_to (bits) ; }
};
}
