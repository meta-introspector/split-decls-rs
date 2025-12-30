// Generated macro for Pattern (enum)
macro_rules! Depcrate_tyPattern {
() => {
// Module: crate::ty
// Provides: {"Pattern"}
// Dependencies: {}
# [doc = " Represents a pattern in the type system"] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum Pattern { Range { start : Option < TyConst > , end : Option < TyConst > , include_end : bool } , }
};
}
