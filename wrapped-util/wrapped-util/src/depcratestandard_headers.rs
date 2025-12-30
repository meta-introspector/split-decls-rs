// Generated macro for standard_headers (macro)
macro_rules! Depcratestandard_headers {
() => {
// Module: crate
// Provides: {"standard_headers"}
// Dependencies: {}
macro_rules ! standard_headers { ($ ($ doc : expr , $ name : expr ;) +) => { const HEADERS : & [(&'static str , &'static str)] = & [$ (($ doc , $ name) ,) +] ; } }
};
}
