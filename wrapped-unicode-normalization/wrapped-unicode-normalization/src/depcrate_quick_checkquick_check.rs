// Generated macro for quick_check (function)
macro_rules! Depcrate_quick_checkquick_check {
() => {
// Module: crate::quick_check
// Provides: {"quick_check"}
// Dependencies: {}
# [inline] fn quick_check < F , I > (s : I , is_allowed : F , stream_safe : bool) -> IsNormalized where I : Iterator < Item = char > , F : Fn (char) -> IsNormalized , { let mut last_cc = 0u8 ; let mut nonstarter_count = 0 ; let mut result = IsNormalized :: Yes ; for ch in s { if ch <= '\x7f' { last_cc = 0 ; nonstarter_count = 0 ; continue ; } let cc = canonical_combining_class (ch) ; if last_cc > cc && cc != 0 { return IsNormalized :: No ; } match is_allowed (ch) { IsNormalized :: Yes => () , IsNormalized :: No => return IsNormalized :: No , IsNormalized :: Maybe => { result = IsNormalized :: Maybe ; } } if stream_safe { let decomp = stream_safe :: classify_nonstarters (ch) ; if nonstarter_count + decomp . leading_nonstarters > stream_safe :: MAX_NONSTARTERS { return IsNormalized :: No ; } if decomp . leading_nonstarters == decomp . decomposition_len { nonstarter_count += decomp . decomposition_len ; } else { nonstarter_count = decomp . trailing_nonstarters ; } } last_cc = cc ; } result }
};
}
