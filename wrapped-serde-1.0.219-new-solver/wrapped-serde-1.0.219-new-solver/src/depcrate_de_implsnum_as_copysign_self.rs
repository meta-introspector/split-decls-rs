// Generated macro for num_as_copysign_self (macro)
macro_rules! Depcrate_de_implsnum_as_copysign_self {
() => {
// Module: crate::de::impls
// Provides: {"num_as_copysign_self"}
// Dependencies: {}
macro_rules ! num_as_copysign_self { ($ ty : ident : $ visit : ident) => { # [inline] fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { # [cfg (any (no_float_copysign , not (feature = "std")))] { Ok (v as Self :: Value) } # [cfg (all (not (no_float_copysign) , feature = "std"))] { let sign = if v . is_sign_positive () { 1.0 } else { - 1.0 } ; Ok ((v as Self :: Value) . copysign (sign)) } } } ; }
};
}
