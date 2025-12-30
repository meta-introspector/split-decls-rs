// Generated macro for __internal_path (macro)
macro_rules! Depcrate_filters_path__internal_path {
() => {
// Module: crate::filters::path
// Provides: {"__internal_path"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __internal_path { (@ start) => ($ crate :: path :: end ()) ; (@ start ..) => ({ compile_error ! ("'..' cannot be the only segment") }) ; (@ start $ first : tt $ (/ $ tail : tt) *) => ({ $ crate :: __internal_path ! (@ munch $ crate :: any () ; [$ first] [$ (/ $ tail) *]) }) ; (@ munch $ sum : expr ; [$ cur : tt] [/ $ next : tt $ (/ $ tail : tt) *]) => ({ $ crate :: __internal_path ! (@ munch $ crate :: Filter :: and ($ sum , $ crate :: __internal_path ! (@ segment $ cur)) ; [$ next] [$ (/ $ tail) *]) }) ; (@ munch $ sum : expr ; [$ cur : tt] []) => ({ $ crate :: __internal_path ! (@ last $ sum ; $ cur) }) ; (@ last $ sum : expr ; ..) => ($ sum) ; (@ last $ sum : expr ; $ end : tt) => ($ crate :: Filter :: and ($ crate :: Filter :: and ($ sum , $ crate :: __internal_path ! (@ segment $ end)) , $ crate :: path :: end ())) ; (@ segment ..) => (compile_error ! ("'..' must be the last segment")) ; (@ segment $ param : ty) => ($ crate :: path :: param ::<$ param > ()) ; (@ segment $ s : literal) => ({ # [derive (Clone , Copy)] struct __StaticPath ; impl :: std :: convert :: AsRef < str > for __StaticPath { fn as_ref (& self) -> & str { static S : & str = $ s ; S } } $ crate :: path (__StaticPath) }) ; }
};
}
