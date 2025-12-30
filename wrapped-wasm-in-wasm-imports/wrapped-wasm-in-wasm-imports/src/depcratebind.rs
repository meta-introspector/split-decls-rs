// Generated macro for bind (function)
macro_rules! Depcratebind {
() => {
// Module: crate
// Provides: {"bind"}
// Dependencies: {}
fn bind (this : & JsValue , func_name : & str) -> Result < () , JsValue > { let property_key = JsValue :: from (func_name) ; let orig_func = Reflect :: get (this , & property_key) ? . dyn_into :: < Function > () ? ; let func = orig_func . bind (this) ; if ! Reflect :: set (this , & property_key , & func) ? { return Err (JsValue :: from ("failed to set property")) ; } Ok (()) }
};
}
