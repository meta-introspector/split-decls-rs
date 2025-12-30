// Generated macro for Field (struct)
macro_rules! Depcrate_internals_attrField {
() => {
// Module: crate::internals::attr
// Provides: {"Field"}
// Dependencies: {}
# [doc = " Represents field attribute information"] pub struct Field { name : MultiName , skip_serializing : bool , skip_deserializing : bool , skip_serializing_if : Option < syn :: ExprPath > , default : Default , serialize_with : Option < syn :: ExprPath > , deserialize_with : Option < syn :: ExprPath > , ser_bound : Option < Vec < syn :: WherePredicate > > , de_bound : Option < Vec < syn :: WherePredicate > > , borrowed_lifetimes : BTreeSet < syn :: Lifetime > , getter : Option < syn :: ExprPath > , flatten : bool , transparent : bool , }
};
}
