// Generated macro for resume_unwind (function)
macro_rules! Depcrate_panickingresume_unwind {
() => {
// Module: crate::panicking
// Provides: {"resume_unwind"}
// Dependencies: {}
# [doc = " This is the entry point for `resume_unwind`."] # [doc = " It just forwards the payload to the panic runtime."] # [cfg_attr (feature = "panic_immediate_abort" , inline)] pub fn resume_unwind (payload : Box < dyn Any + Send >) -> ! { panic_count :: increase (false) ; struct RewrapBox (Box < dyn Any + Send >) ; unsafe impl PanicPayload for RewrapBox { fn take_box (& mut self) -> * mut (dyn Any + Send) { Box :: into_raw (mem :: replace (& mut self . 0 , Box :: new (()))) } fn get (& mut self) -> & (dyn Any + Send) { & * self . 0 } } impl fmt :: Display for RewrapBox { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (payload_as_str (& self . 0)) } } rust_panic (& mut RewrapBox (payload)) }
};
}
