// Generated macro for impl_279 (impl)
macro_rules! Depcrate_spanimpl_279 {
() => {
// Module: crate::span
// Provides: {"impl_279"}
// Dependencies: {}
impl < 'a > Record < 'a > { # [doc = " Constructs a new `Record` from a `ValueSet`."] pub fn new (values : & 'a field :: ValueSet < 'a >) -> Self { Self { values } } # [doc = " Records all the fields in this `Record` with the provided [Visitor]."] # [doc = ""] # [doc = " [visitor]: super::field::Visit"] pub fn record (& self , visitor : & mut dyn field :: Visit) { self . values . record (visitor) } # [doc = " Returns the number of fields that would be visited from this `Record`"] # [doc = " when [`Record::record()`] is called"] # [doc = ""] # [doc = " [`Record::record()`]: Record::record()"] pub fn len (& self) -> usize { self . values . len () } # [doc = " Returns `true` if this `Record` contains a value for the given `Field`."] pub fn contains (& self , field : & field :: Field) -> bool { self . values . contains (field) } # [doc = " Returns true if this `Record` contains _no_ values."] pub fn is_empty (& self) -> bool { self . values . is_empty () } }
};
}
