// Generated macro for impl_const_constructors (macro)
macro_rules! Depcrate_ule_plainimpl_const_constructors {
() => {
// Module: crate::ule::plain
// Provides: {"impl_const_constructors"}
// Dependencies: {}
macro_rules ! impl_const_constructors { ($ base : ty , $ size : literal) => { impl ZeroSlice <$ base > { # [doc = " This function can be used for constructing ZeroVecs in a const context, avoiding"] # [doc = " parsing checks."] # [doc = ""] # [doc = " This cannot be generic over T because of current limitations in `const`, but if"] # [doc = " this method is needed in a non-const context, check out [`ZeroSlice::parse_bytes()`]"] # [doc = " instead."] # [doc = ""] # [doc = " See [`ZeroSlice::cast()`] for an example."] pub const fn try_from_bytes (bytes : & [u8]) -> Result <& Self , UleError > { let len = bytes . len () ; # [allow (clippy :: modulo_one)] if len % $ size == 0 { Ok (unsafe { Self :: from_bytes_unchecked (bytes) }) } else { Err (UleError :: InvalidLength { ty : concat ! ("<const construct: " , $ size , ">") , len , }) } } } } ; }
};
}
