// Generated macro for macro_122 (macro)
macro_rules! Depcrate_set_header_responsemacro_122 {
() => {
// Module: crate::set_header::response
// Provides: {"macro_122"}
// Dependencies: {}
pin_project ! { # [doc = " Response future for [`SetResponseHeader`]."] # [derive (Debug)] pub struct ResponseFuture < F , M > { # [pin] future : F , header_name : HeaderName , make : M , mode : InsertHeaderMode , } }
};
}
