// Generated macro for impl_307 (impl)
macro_rules! Depcrate_http3_driverimpl_307 {
() => {
// Module: crate::http3::driver
// Provides: {"impl_307"}
// Dependencies: {}
impl < C , T : Into < C > > RequestSender < C , T > { # [doc = " Send a request to the [H3Driver]. This can only fail if the driver is"] # [doc = " gone."] # [inline (always)] pub fn send (& self , v : T) -> Result < () , mpsc :: error :: SendError < C > > { self . sender . send (v . into ()) } }
};
}
