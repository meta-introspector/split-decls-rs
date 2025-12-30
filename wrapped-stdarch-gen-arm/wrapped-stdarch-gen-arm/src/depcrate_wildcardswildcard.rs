// Generated macro for Wildcard (enum)
macro_rules! Depcrate_wildcardsWildcard {
() => {
// Module: crate::wildcards
// Provides: {"Wildcard"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash , SerializeDisplay , DeserializeFromStr)] pub enum Wildcard { Type (Option < usize >) , # [doc = " NEON type derivated by a base type"] NEONType (Option < usize > , Option < VectorTupleSize > , Option < SuffixKind >) , # [doc = " SVE type derivated by a base type"] SVEType (Option < usize > , Option < VectorTupleSize >) , # [doc = " Integer representation of bitsize"] Size (Option < usize >) , # [doc = " Integer representation of bitsize minus one"] SizeMinusOne (Option < usize >) , # [doc = " Literal representation of the bitsize: b(yte), h(half), w(ord) or d(ouble)"] SizeLiteral (Option < usize >) , # [doc = " Literal representation of the type kind: f(loat), s(igned), u(nsigned)"] TypeKind (Option < usize > , Option < TypeKindOptions >) , # [doc = " Log2 of the size in bytes"] SizeInBytesLog2 (Option < usize >) , # [doc = " Predicate to be inferred from the specified type"] Predicate (Option < usize >) , # [doc = " Predicate to be inferred from the greatest type"] MaxPredicate , Scale (Box < Wildcard > , Box < TypeKind >) , LLVMLink , NVariant , # [doc = " Predicate forms to use and placeholder for a predicate form function name modifier"] PredicateForms (PredicationMask) , # [doc = " User-set wildcard through `substitutions`"] Custom (String) , }
};
}
