// Generated macro for Select (struct)
macro_rules! Depcrate_hedge_selectSelect {
() => {
// Module: crate::hedge::select
// Provides: {"Select"}
// Dependencies: {}
# [doc = " Select is a middleware which attempts to clone the request and sends the"] # [doc = " original request to the A service and, if the request was able to be cloned,"] # [doc = " the cloned request to the B service.  Both resulting futures will be polled"] # [doc = " and whichever future completes first will be used as the result."] # [derive (Debug)] pub struct Select < P , A , B > { policy : P , a : A , b : B , }
};
}
