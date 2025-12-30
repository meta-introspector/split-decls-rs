// Generated macro for Input (struct)
macro_rules! Depcrate_inputInput {
() => {
// Module: crate::input
// Provides: {"Input"}
// Dependencies: {}
# [doc = " A wrapper around `&'a [u8]` that helps in writing panic-free code."] # [doc = ""] # [doc = " No methods of `Input` will ever panic."] # [doc = ""] # [doc = " Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit"] # [doc = " non-constant-time comparisons."] # [derive (Clone , Copy)] pub struct Input < 'a > { value : no_panic :: Slice < 'a > , }
};
}
