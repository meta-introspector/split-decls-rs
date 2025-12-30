// Generated macro for version (macro)
macro_rules! Depcrate_format_description_parseversion {
() => {
// Module: crate::format_description::parse
// Provides: {"version"}
// Dependencies: {}
# [doc = " A helper macro to make version restrictions simpler to read and write."] macro_rules ! version { ($ range : expr) => { $ range . contains (& VERSION) } ; }
};
}
