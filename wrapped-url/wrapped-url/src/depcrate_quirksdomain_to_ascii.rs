// Generated macro for domain_to_ascii (function)
macro_rules! Depcrate_quirksdomain_to_ascii {
() => {
// Module: crate::quirks
// Provides: {"domain_to_ascii"}
// Dependencies: {}
# [doc = " <https://url.spec.whatwg.org/#dom-url-domaintoascii>"] pub fn domain_to_ascii (domain : & str) -> String { match Host :: parse (domain) { Ok (Host :: Domain (domain)) => domain , _ => String :: new () , } }
};
}
