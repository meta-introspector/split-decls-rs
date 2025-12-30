// Generated macro for ValueSet (struct)
macro_rules! Depcrate_fieldValueSet {
() => {
// Module: crate::field
// Provides: {"ValueSet"}
// Dependencies: {}
# [doc = " A set of fields and values for a span."] pub struct ValueSet < 'a > { values : & 'a [(& 'a Field , Option < & 'a (dyn Value + 'a) >)] , fields : & 'a FieldSet , }
};
}
