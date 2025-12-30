// Generated macro for macro_783 (macro)
macro_rules! Depcrate_util_call_all_unorderedmacro_783 {
() => {
// Module: crate::util::call_all::unordered
// Provides: {"macro_783"}
// Dependencies: {}
pin_project ! { # [doc = " A stream of responses received from the inner service in received order."] # [doc = ""] # [doc = " Similar to [`CallAll`] except, instead of yielding responses in request order,"] # [doc = " responses are returned as they are available."] # [doc = ""] # [doc = " [`CallAll`]: crate::util::CallAll"] # [derive (Debug)] pub struct CallAllUnordered < Svc , S > where Svc : Service < S :: Item >, S : Stream , { # [pin] inner : common :: CallAll < Svc , S , FuturesUnordered < Svc :: Future >>, } }
};
}
