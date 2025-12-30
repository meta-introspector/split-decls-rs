// Generated macro for impl_335 (impl)
macro_rules! Depcrateimpl_335 {
() => {
// Module: crate
// Provides: {"impl_335"}
// Dependencies: {}
impl Drop for JsValue { # [inline] fn drop (& mut self) { unsafe { debug_assert ! (self . idx >= __rt :: JSIDX_OFFSET , "free of stack slot {}" , self . idx) ; if self . idx >= __rt :: JSIDX_RESERVED { __wbindgen_object_drop_ref (self . idx) ; } } } }
};
}
