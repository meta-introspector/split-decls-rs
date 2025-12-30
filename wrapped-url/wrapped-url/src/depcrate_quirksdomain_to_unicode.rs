// Generated macro for domain_to_unicode (function)
macro_rules! Depcrate_quirksdomain_to_unicode {
() => {
// Module: crate::quirks
// Provides: {"domain_to_unicode"}
// Dependencies: {}
# [doc = " <https://url.spec.whatwg.org/#dom-url-domaintounicode>"] pub fn domain_to_unicode (domain : & str) -> String { match Host :: parse (domain) { Ok (Host :: Domain (ref domain)) => { let (unicode , _errors) = idna :: domain_to_unicode (domain) ; unicode } _ => String :: new () , } }
};
}
