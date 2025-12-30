// Generated macro for lookup_unassigned (function)
macro_rules! Depcrate_bidi_classlookup_unassigned {
() => {
// Module: crate::bidi_class
// Provides: {"lookup_unassigned"}
// Dependencies: {}
# [doc = " Look up a code point in the unassigned default Bidi classes."] fn lookup_unassigned < 'a > (codepoint : u32 , defaults : & [(u32 , u32 , & 'a str)] ,) -> Option < & 'a str > { defaults . iter () . find (| & & (start , end , _) | start <= codepoint && codepoint <= end) . map (| & (_ , _ , bidi_class) | bidi_class) }
};
}
