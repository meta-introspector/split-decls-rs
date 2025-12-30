// Generated macro for has_alphanumeric (function)
macro_rules! Depcrate_wordhas_alphanumeric {
() => {
// Module: crate::word
// Provides: {"has_alphanumeric"}
// Dependencies: {}
# [inline] fn has_alphanumeric (s : & & str) -> bool { use crate :: tables :: util :: is_alphanumeric ; s . chars () . any (is_alphanumeric) }
};
}
