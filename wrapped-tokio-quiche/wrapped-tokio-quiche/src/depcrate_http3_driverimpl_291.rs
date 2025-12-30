// Generated macro for impl_291 (impl)
macro_rules! Depcrate_http3_driverimpl_291 {
() => {
// Module: crate::http3::driver
// Provides: {"impl_291"}
// Dependencies: {}
impl fmt :: Display for H3ConnectionError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let s : & dyn fmt :: Display = match self { Self :: ControllerWentAway => & "controller went away" , Self :: H3 (e) => e , Self :: GoAway => & "goaway" , Self :: NonexistentStream => & "nonexistent stream" , Self :: PostAcceptTimeout => & "post accept timeout hit" , } ; write ! (f , "H3ConnectionError: {s}") } }
};
}
