// Generated macro for AssocKind (enum)
macro_rules! Depcrate_tyAssocKind {
() => {
// Module: crate::ty
// Provides: {"AssocKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AssocKind { Const { name : Symbol } , Fn { name : Symbol , has_self : bool } , Type { data : AssocTypeData } , }
};
}
