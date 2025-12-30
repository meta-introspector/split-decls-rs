// Generated macro for WasmRet (struct)
macro_rules! Depcrate_convert_traitsWasmRet {
() => {
// Module: crate::convert::traits
// Provides: {"WasmRet"}
// Dependencies: {}
# [doc = " A repr(C) struct containing all of the primitives of a `WasmAbi` type, in"] # [doc = " order."] # [doc = ""] # [doc = " This is used as the return type of imported/exported functions. `WasmAbi`"] # [doc = " types aren't guaranteed to be FFI-safe, so we can't return them directly:"] # [doc = " instead we return this."] # [doc = ""] # [doc = " If all but one of the primitives is `()`, this corresponds to returning the"] # [doc = " remaining primitive directly, otherwise a return pointer is used."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] # [repr (C)] pub struct WasmRet < T : WasmAbi > { prim1 : T :: Prim1 , prim2 : T :: Prim2 , prim3 : T :: Prim3 , prim4 : T :: Prim4 , }
};
}
