// Generated macro for extend_integer_width (function)
macro_rules! Depcrate_callconv_riscvextend_integer_width {
() => {
// Module: crate::callconv::riscv
// Provides: {"extend_integer_width"}
// Dependencies: {}
fn extend_integer_width < Ty > (arg : & mut ArgAbi < '_ , Ty > , xlen : u64) { if let BackendRepr :: Scalar (scalar) = arg . layout . backend_repr && let Primitive :: Int (i , _) = scalar . primitive () && i . size () . bits () == 32 && xlen > 32 && let PassMode :: Direct (ref mut attrs) = arg . mode { attrs . ext (ArgExtension :: Sext) ; return ; } arg . extend_integer_width_to (xlen) ; }
};
}
