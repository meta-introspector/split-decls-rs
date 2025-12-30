// Generated macro for TypeKind (enum)
macro_rules! Depcrate_typekindsTypeKind {
() => {
// Module: crate::typekinds
// Provides: {"TypeKind"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash , SerializeDisplay , DeserializeFromStr)] pub enum TypeKind { Vector (VectorType) , Base (BaseType) , Pointer (Box < TypeKind > , AccessLevel) , Custom (String) , Wildcard (Wildcard) , }
};
}
