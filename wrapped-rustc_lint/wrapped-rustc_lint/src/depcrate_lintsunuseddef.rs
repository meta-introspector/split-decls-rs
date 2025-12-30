// Generated macro for UnusedDef (struct)
macro_rules! Depcrate_lintsUnusedDef {
() => {
// Module: crate::lints
// Provides: {"UnusedDef"}
// Dependencies: {}
pub (crate) struct UnusedDef < 'a , 'b > { pub pre : & 'a str , pub post : & 'a str , pub cx : & 'a LateContext < 'b > , pub def_id : DefId , pub note : Option < Symbol > , pub suggestion : Option < UnusedDefSuggestion > , }
};
}
