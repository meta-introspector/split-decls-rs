// Generated macro for Variant (struct)
macro_rules! Depcrate_internals_attrVariant {
() => {
// Module: crate::internals::attr
// Provides: {"Variant"}
// Dependencies: {}
# [doc = " Represents variant attribute information"] pub struct Variant { name : MultiName , rename_all_rules : RenameAllRules , ser_bound : Option < Vec < syn :: WherePredicate > > , de_bound : Option < Vec < syn :: WherePredicate > > , skip_deserializing : bool , skip_serializing : bool , other : bool , serialize_with : Option < syn :: ExprPath > , deserialize_with : Option < syn :: ExprPath > , borrow : Option < BorrowAttribute > , untagged : bool , }
};
}
