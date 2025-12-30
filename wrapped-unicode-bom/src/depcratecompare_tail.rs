// Generated macro for compare_tail (macro)
macro_rules! Depcratecompare_tail {
() => {
// Module: crate
// Provides: {"compare_tail"}
// Dependencies: {}
macro_rules ! compare_tail { ($ slice : ident , $ bytes : expr) => { compare_tail ! ($ slice , $ bytes , 1) } ; ($ slice : ident , $ bytes : expr , $ from : expr) => { compare_tail ! ($ slice , $ bytes . len () + $ from , $ bytes , $ from) } ; ($ slice : ident , $ len : expr , $ bytes : expr , $ from : expr) => { $ slice . len () >= $ len && $ slice [$ from ..$ from + $ bytes . len ()] == $ bytes } ; }
};
}
