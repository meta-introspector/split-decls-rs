// Generated macro for impl_296 (impl)
macro_rules! Depcrate_http3_driverimpl_296 {
() => {
// Module: crate::http3::driver
// Provides: {"impl_296"}
// Dependencies: {}
impl H3Event { # [doc = " Generates an event from an applicable [`H3ConnectionError`]."] fn from_error (err : & H3ConnectionError) -> Option < Self > { Some (match err { H3ConnectionError :: H3 (e) => Self :: ConnectionError (* e) , H3ConnectionError :: PostAcceptTimeout => Self :: ConnectionShutdown (Some (H3ConnectionError :: PostAcceptTimeout) ,) , _ => return None , }) } }
};
}
