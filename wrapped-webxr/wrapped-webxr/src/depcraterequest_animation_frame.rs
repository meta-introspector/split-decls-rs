// Generated macro for request_animation_frame (function)
macro_rules! Depcraterequest_animation_frame {
() => {
// Module: crate
// Provides: {"request_animation_frame"}
// Dependencies: {}
fn request_animation_frame (session : & XrSession , f : & Closure < dyn FnMut (f64 , XrFrame) >) -> u32 { session . request_animation_frame (f . as_ref () . unchecked_ref ()) }
};
}
