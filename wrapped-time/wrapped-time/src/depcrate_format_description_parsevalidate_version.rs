// Generated macro for validate_version (macro)
macro_rules! Depcrate_format_description_parsevalidate_version {
() => {
// Module: crate::format_description::parse
// Provides: {"validate_version"}
// Dependencies: {}
# [doc = " A helper macro to statically validate the version (when used as a const parameter)."] macro_rules ! validate_version { ($ version : ident) => { const { assert ! ($ version >= 1 && $ version <= 2) ; } } ; }
};
}
