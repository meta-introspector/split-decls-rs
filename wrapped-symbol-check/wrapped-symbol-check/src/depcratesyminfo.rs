// Generated macro for SymInfo (struct)
macro_rules! DepcrateSymInfo {
() => {
// Module: crate
// Provides: {"SymInfo"}
// Dependencies: {}
# [doc = " Information collected from `object`, for convenience."] # [expect (unused)] # [derive (Clone , Debug)] struct SymInfo { name : String , kind : SymbolKind , scope : SymbolScope , section : String , is_undefined : bool , is_global : bool , is_local : bool , is_weak : bool , is_common : bool , address : u64 , object : String , }
};
}
