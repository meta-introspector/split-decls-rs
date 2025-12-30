// Generated macro for impl_113 (impl)
macro_rules! Depcrate_convert_implsimpl_113 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_113"}
// Dependencies: {}
impl < T , E > ReturnWasmAbi for Result < T , E > where T : IntoWasmAbi , E : Into < JsValue > , T :: Abi : WasmAbi < Prim3 = () , Prim4 = () > , { type Abi = Result < T :: Abi , u32 > ; # [inline] fn return_abi (self) -> Self :: Abi { match self { Ok (v) => Ok (v . into_abi ()) , Err (e) => { let jsval = e . into () ; Err (jsval . into_abi ()) } } } }
};
}
