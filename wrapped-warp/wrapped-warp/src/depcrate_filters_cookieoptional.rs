// Generated macro for optional (function)
macro_rules! Depcrate_filters_cookieoptional {
() => {
// Module: crate::filters::cookie
// Provides: {"optional"}
// Dependencies: {}
# [doc = " Creates a `Filter` that looks for an optional cookie by name."] # [doc = ""] # [doc = " If found, extracts the value of the cookie, otherwise continues"] # [doc = " the request, extracting `None`."] pub fn optional < T > (name : & 'static str ,) -> impl Filter < Extract = One < Option < T > > , Error = Infallible > + Copy where T : FromStr + Send + 'static , { header :: optional2 () . map (move | opt : Option < Cookie > | { let cookie = opt . and_then (| cookie | cookie . get (name) . map (| x | T :: from_str (x))) ; match cookie { Some (Ok (t)) => Some (t) , Some (Err (_)) => None , None => None , } }) }
};
}
