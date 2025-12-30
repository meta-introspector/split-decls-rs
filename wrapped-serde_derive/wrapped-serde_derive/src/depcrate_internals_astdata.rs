// Generated macro for Data (enum)
macro_rules! Depcrate_internals_astData {
() => {
// Module: crate::internals::ast
// Provides: {"Data"}
// Dependencies: {}
# [doc = " The fields of a struct or enum."] # [doc = ""] # [doc = " Analogous to `syn::Data`."] pub enum Data < 'a > { Enum (Vec < Variant < 'a > >) , Struct (Style , Vec < Field < 'a > >) , }
};
}
