// Generated macro for pluralize (macro)
macro_rules! Depcratepluralize {
() => {
// Module: crate
// Provides: {"pluralize"}
// Dependencies: {}
# [macro_export] macro_rules ! pluralize { ($ x : expr) => { if $ x == 1 { "" } else { "s" } } ; ("has" , $ x : expr) => { if $ x == 1 { "has" } else { "have" } } ; ("is" , $ x : expr) => { if $ x == 1 { "is" } else { "are" } } ; ("was" , $ x : expr) => { if $ x == 1 { "was" } else { "were" } } ; ("this" , $ x : expr) => { if $ x == 1 { "this" } else { "these" } } ; }
};
}
