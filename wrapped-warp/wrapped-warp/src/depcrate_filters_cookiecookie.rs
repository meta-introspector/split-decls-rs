// Generated macro for cookie (function)
macro_rules! Depcrate_filters_cookiecookie {
() => {
// Module: crate::filters::cookie
// Provides: {"cookie"}
// Dependencies: {}
# [doc = " Creates a `Filter` that requires a cookie by name."] # [doc = ""] # [doc = " If found, extracts the value of the cookie, otherwise rejects."] pub fn cookie < T > (name : & 'static str) -> impl Filter < Extract = One < T > , Error = Rejection > + Copy where T : FromStr + Send + 'static , { header :: header2 () . and_then (move | cookie : Cookie | { let cookie = cookie . get (name) . ok_or_else (| | crate :: reject :: missing_cookie (name)) . and_then (| s | T :: from_str (s) . map_err (| _ | crate :: reject :: missing_cookie (name))) ; future :: ready (cookie) }) }
};
}
