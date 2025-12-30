// Generated macro for into_linger (function)
macro_rules! Depcrate_socketinto_linger {
() => {
// Module: crate::socket
// Provides: {"into_linger"}
// Dependencies: {}
const fn into_linger (duration : Option < Duration >) -> sys :: linger { match duration { Some (duration) => sys :: linger { l_onoff : 1 , l_linger : duration . as_secs () as _ , } , None => sys :: linger { l_onoff : 0 , l_linger : 0 , } , } }
};
}
