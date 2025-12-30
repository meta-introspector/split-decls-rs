// Generated macro for macro_675 (macro)
macro_rules! Depcrate_follow_redirectmacro_675 {
() => {
// Module: crate::follow_redirect
// Provides: {"macro_675"}
// Dependencies: {}
pin_project ! { # [doc = " Response future for [`FollowRedirect`]."] # [derive (Debug)] pub struct ResponseFuture < S , B , P > where S : Service < Request < B >>, { # [pin] future : Either < S :: Future , Oneshot < S , Request < B >>>, service : S , policy : P , method : Method , uri : Uri , version : Version , headers : HeaderMap < HeaderValue >, body : BodyRepr < B >, } }
};
}
